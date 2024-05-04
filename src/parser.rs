use crate::{
	ast::{
		Ast, BinaryOperator, Definition, Expression, ExpressionKind, Field, Parameter, Procedure, Statement,
		StatementKind, Struct, Ty, TyKind,
	},
	lexer::{lex, Loc, Token, TokenKind},
};
use std::{cell::Cell, path::PathBuf};

const INITIAL_FUEL: u8 = 255;

pub fn parse(input: &str, file: PathBuf) -> Ast {
	let tokens = lex(input, file);
	Parser::new(tokens).parse()
}

struct Parser {
	tokens: Vec<Token>,
	cursor: usize,
	fuel: Cell<u8>,
}

impl Parser {
	fn new(tokens: Vec<Token>) -> Parser {
		Parser { tokens, cursor: 0, fuel: Cell::new(INITIAL_FUEL) }
	}

	fn parse(mut self) -> Ast {
		let mut definitions = Vec::new();

		while !self.at_eof() {
			definitions.push(self.parse_definition());
		}

		Ast { definitions }
	}

	fn parse_definition(&mut self) -> Definition {
		match self.current() {
			TokenKind::ProcKw => self.parse_procedure(),
			TokenKind::StructKw => self.parse_struct(),
			_ => self.error("expected definition".to_string()),
		}
	}

	fn parse_procedure(&mut self) -> Definition {
		self.bump(TokenKind::ProcKw);
		let name = self.expect_text(TokenKind::Identifier);

		self.expect(TokenKind::LParen);
		let mut parameters = Vec::new();

		while !self.at_eof() && !self.at(TokenKind::RParen) {
			let parameter_name = self.expect_text(TokenKind::Identifier);
			let parameter_ty = self.parse_ty();
			parameters.push(Parameter { name: parameter_name, ty: parameter_ty });

			match (self.current(), self.lookahead()) {
				(TokenKind::RParen, _) => {}
				(TokenKind::Comma, TokenKind::RParen) => self.bump(TokenKind::Comma),
				_ => self.expect(TokenKind::Comma),
			}

			if self.at(TokenKind::RParen) {
				self.eat(TokenKind::Comma);
			}
		}

		self.expect(TokenKind::RParen);

		let mut return_ty = None;
		if !self.at(TokenKind::LBrace) {
			return_ty = Some(self.parse_ty());
		}

		let body = self.parse_block();

		Definition::Procedure(Procedure { name, parameters, return_ty, body })
	}

	fn parse_struct(&mut self) -> Definition {
		self.bump(TokenKind::StructKw);
		let name = self.expect_text(TokenKind::Identifier);
		self.expect(TokenKind::LBrace);
		let mut fields = Vec::new();

		while !self.at_eof() && !self.at(TokenKind::RBrace) {
			let field_name = self.expect_text(TokenKind::Identifier);
			let field_ty = self.parse_ty();
			fields.push(Field { name: field_name, ty: field_ty });
		}

		self.expect(TokenKind::RBrace);

		Definition::Struct(Struct { name, fields })
	}

	fn parse_statement(&mut self) -> Statement {
		match self.current() {
			TokenKind::VarKw => self.parse_local_declaration(),
			TokenKind::ReturnKw => self.parse_return(),
			TokenKind::LBrace => self.parse_block(),
			_ if self.lookahead() == TokenKind::ColonEqual => self.parse_local_definition(),
			_ => {
				if self.lookahead() == TokenKind::ColonEqual {
					return self.parse_local_declaration();
				}

				let loc = self.current_loc();
				let lhs = self.parse_expression();
				let operator_kind = self.current();
				let operator = match assignment_token_kind_to_operator(operator_kind) {
					Some(op) => op,
					None => return Statement { kind: StatementKind::Expression(lhs), loc },
				};
				self.bump(operator_kind);

				let rhs = self.parse_expression();

				match operator {
					Some(operator) => Statement {
						kind: StatementKind::Assignment {
							lhs: lhs.clone(),
							rhs: Expression {
								kind: ExpressionKind::Binary { lhs: Box::new(lhs), operator, rhs: Box::new(rhs) },
								loc: loc.clone(),
							},
						},
						loc,
					},
					None => Statement { kind: StatementKind::Assignment { lhs, rhs }, loc },
				}
			}
		}
	}

	fn parse_local_declaration(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::VarKw);
		let name = self.expect_text(TokenKind::Identifier);
		let ty = self.parse_ty();
		Statement { kind: StatementKind::LocalDeclaration { name, ty }, loc }
	}

	fn parse_local_definition(&mut self) -> Statement {
		let loc = self.current_loc();
		let name = self.expect_text(TokenKind::Identifier);
		self.bump(TokenKind::ColonEqual);

		let value = self.parse_expression();

		Statement { kind: StatementKind::LocalDefinition { name, value }, loc }
	}

	fn parse_return(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::ReturnKw);
		let value = if self.at_expression() { Some(self.parse_expression()) } else { None };

		Statement { kind: StatementKind::Return { value }, loc }
	}

	fn parse_block(&mut self) -> Statement {
		let loc = self.current_loc();
		self.bump(TokenKind::LBrace);

		let mut statements = Vec::new();
		while !self.at_eof() && !self.at(TokenKind::RBrace) {
			statements.push(self.parse_statement());
		}

		self.expect(TokenKind::RBrace);

		Statement { kind: StatementKind::Block(statements), loc }
	}

	fn parse_expression(&mut self) -> Expression {
		self.parse_expression_bp(0)
	}

	fn parse_expression_bp(&mut self, bp: u8) -> Expression {
		let mut lhs = self.parse_atom();

		loop {
			let operator_kind = self.current();
			let operator = match token_kind_to_operator(operator_kind) {
				Some(o) => o,
				None => break,
			};
			let right_bp = operator_to_bp(operator);

			if right_bp <= bp {
				break;
			}
			self.bump(operator_kind);
			let loc = self.current_loc();
			lhs = Expression {
				kind: ExpressionKind::Binary {
					lhs: Box::new(lhs),
					operator,
					rhs: Box::new(self.parse_expression_bp(right_bp)),
				},
				loc,
			}
		}
		lhs
	}

	fn parse_atom(&mut self) -> Expression {
		assert!(self.at_expression());
		match self.current() {
			TokenKind::Integer => {
				let loc = self.current_loc();
				let text = self.expect_text(TokenKind::Integer);
				Expression { kind: ExpressionKind::Integer(text.parse().unwrap()), loc }
			}
			TokenKind::Identifier => {
				let loc = self.current_loc();
				let text = self.expect_text(TokenKind::Identifier);
				Expression { kind: ExpressionKind::Variable(text), loc }
			}
			TokenKind::TrueKw => {
				let loc = self.current_loc();
				self.bump(TokenKind::TrueKw);
				Expression { kind: ExpressionKind::True, loc }
			}

			TokenKind::FalseKw => {
				let loc = self.current_loc();
				self.bump(TokenKind::FalseKw);
				Expression { kind: ExpressionKind::False, loc }
			}
			_ => self.error("expected expression".to_string()),
		}
	}

	fn parse_ty(&mut self) -> Ty {
		let loc = self.current_loc();
		let text = self.expect_text(TokenKind::Identifier);
		match text.as_str() {
			"int" => Ty { kind: TyKind::Int, loc },
			_ => self.error("expected type".to_string()),
		}
	}

	fn expect_text(&mut self, kind: TokenKind) -> String {
		let text = self.tokens[self.cursor].text.clone();
		self.expect(kind);
		text
	}

	fn expect(&mut self, kind: TokenKind) {
		if !self.eat(kind) {
			self.error(format!("expected {kind:?} but found {:?}", self.current()));
		}
	}

	fn eat(&mut self, kind: TokenKind) -> bool {
		if self.at(kind) {
			self.bump(kind);
			return true;
		}
		false
	}

	fn bump(&mut self, kind: TokenKind) {
		assert!(self.at(kind));
		self.cursor += 1;
		self.fuel.set(INITIAL_FUEL);
	}

	fn at_expression(&self) -> bool {
		matches!(self.current(), TokenKind::Integer | TokenKind::Identifier | TokenKind::TrueKw | TokenKind::FalseKw)
	}

	fn at(&self, kind: TokenKind) -> bool {
		self.current() == kind
	}

	fn current(&self) -> TokenKind {
		let remaining_fuel = self.fuel.get();
		if remaining_fuel == 0 {
			panic!("parser ran out of fuel");
		}
		self.fuel.set(remaining_fuel - 1);

		if self.at_eof() {
			return TokenKind::Eof;
		}
		self.tokens[self.cursor].kind
	}

	fn current_loc(&self) -> Loc {
		self.tokens[self.cursor].loc.clone()
	}

	fn lookahead(&self) -> TokenKind {
		if self.cursor + 1 >= self.tokens.len() {
			return TokenKind::Eof;
		}
		self.tokens[self.cursor + 1].kind
	}

	fn at_eof(&self) -> bool {
		self.cursor >= self.tokens.len()
	}

	fn error(&self, msg: String) -> ! {
		let loc = if self.at_eof() {
			self.tokens[self.tokens.len() - 1].loc.clone()
		} else {
			self.tokens[self.cursor].loc.clone()
		};

		crate::error(loc, msg);
	}
}

fn token_kind_to_operator(kind: TokenKind) -> Option<BinaryOperator> {
	Some(match kind {
		TokenKind::Plus => BinaryOperator::Add,
		TokenKind::Minus => BinaryOperator::Subtract,
		TokenKind::Star => BinaryOperator::Multiply,
		TokenKind::Slash => BinaryOperator::Divide,
		TokenKind::Percent => BinaryOperator::Modulo,
		TokenKind::LessLess => BinaryOperator::ShiftLeft,
		TokenKind::GreaterGreater => BinaryOperator::ShiftRight,
		TokenKind::And => BinaryOperator::BitAnd,
		TokenKind::Pipe => BinaryOperator::BitOr,
		TokenKind::Caret => BinaryOperator::BitXor,
		TokenKind::AndAnd => BinaryOperator::And,
		TokenKind::PipePipe => BinaryOperator::Or,
		TokenKind::EqualEqual => BinaryOperator::Equal,
		TokenKind::BangEqual => BinaryOperator::NotEqual,
		TokenKind::Less => BinaryOperator::Less,
		TokenKind::Greater => BinaryOperator::Greater,
		TokenKind::LessEqual => BinaryOperator::LessEqual,
		TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
		_ => return None,
	})
}

fn operator_to_bp(operator: BinaryOperator) -> u8 {
	match operator {
		BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => 9,

		BinaryOperator::BitAnd => 8,

		BinaryOperator::BitXor => 7,

		BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 6,

		BinaryOperator::Add | BinaryOperator::Subtract => 5,

		BinaryOperator::BitOr => 4,

		BinaryOperator::Equal
		| BinaryOperator::NotEqual
		| BinaryOperator::Less
		| BinaryOperator::Greater
		| BinaryOperator::LessEqual
		| BinaryOperator::GreaterEqual => 3,

		BinaryOperator::And => 2,

		BinaryOperator::Or => 1,
	}
}

fn assignment_token_kind_to_operator(kind: TokenKind) -> Option<Option<BinaryOperator>> {
	if kind == TokenKind::Equal {
		return Some(None);
	}

	let operator = match kind {
		TokenKind::PlusEqual => BinaryOperator::Add,
		TokenKind::MinusEqual => BinaryOperator::Subtract,
		TokenKind::StarEqual => BinaryOperator::Multiply,
		TokenKind::SlashEqual => BinaryOperator::Divide,
		TokenKind::PercentEqual => BinaryOperator::Modulo,
		TokenKind::LessLessEqual => BinaryOperator::ShiftLeft,
		TokenKind::GreaterGreaterEqual => BinaryOperator::ShiftRight,
		TokenKind::AndEqual => BinaryOperator::BitAnd,
		TokenKind::PipeEqual => BinaryOperator::BitOr,
		TokenKind::CaretEqual => BinaryOperator::BitXor,
		TokenKind::AndAndEqual => BinaryOperator::And,
		TokenKind::PipePipeEqual => BinaryOperator::Or,
		_ => return None,
	};

	Some(Some(operator))
}

#[cfg(test)]
#[test]
fn tests() {
	crate::testing::run_tests("tests/parser", |input| {
		let ast = parse(input, PathBuf::from("test"));
		ast.pretty_print()
	});
}
