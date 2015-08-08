// Copyright 2015 Caleb Jones

#include "tokenizer.hpp"

#include <sstream>
#include <string>
#include <cctype>
#include <set>

#include "token/all_tokens.hpp"

std::set<std::string> operators = {
  "+", "=", "-", "*", "/", "%", "->", "<", ">",
  "<=", ">=", "==", "!=", "+=", "-=", "*=", "/=",
  "//", // Comment operator kludge
};

std::set<char> operator_prefixes = {
  '-', '<', '>', '=', '!', '+', '-', '*', '/',
};

void Tokenizer::advance_char()
{
  if (input.eof()) return;
  if (current_char == '\n') {
    line += 1;
    col = 0;
  }
  col += 1;
  current_char = input.get();
}

char Tokenizer::peek_char()
{
  return current_char;
}

void Tokenizer::advance()
{
  // Return if we're already at EOF
  if (current_token && current_token->type == EOFTOKEN) return;
  // Or if we just got to EOF
  if (input.eof()) {
    current_token = std::make_shared<EOFToken>(line, col);
    return;
  }
  
  // Skip over whitespace
  while (isspace(peek_char())) {
    advance_char();
    // If we hit EOF, then stop trying to advance
    if (input.eof()) {
      current_token = std::make_shared<EOFToken>(line, col);
      return;
    }
  }

  if (isalpha(peek_char()) || peek_char() == '_') {
    // [_a-zA-Z][_a-zA-Z0-9]+
    std::string ident = "";
    while (isalnum(peek_char()) || peek_char() == '_') {
      ident += peek_char();
      advance_char();
    } 
    current_token = std::make_shared<IdentToken>(line, col, ident);
  } else if (isdigit(peek_char())) {
    // [0-9]+ (. [0-9]*)?
    std::string num = "";
    while (isdigit(peek_char())) {
      num += peek_char();
      advance_char();
    }

    if (peek_char() == '.') {
      num += '.';
      advance_char();
      while (isdigit(peek_char())) {
	num += peek_char();
	advance_char();
      }
    }
    current_token = std::make_shared<NumToken>(line, col, num);
  } else {
    current_token = std::make_shared<CharToken>(line, col, peek_char());

    std::string op = "";
    char first_char = peek_char();
    op += first_char;
    advance_char();
    if (operator_prefixes.find(first_char) != operator_prefixes.end()) {
      std::string attempt_op = op;
      attempt_op += peek_char();
      if (operators.find(attempt_op) != operators.end()) {
	op = attempt_op;
	advance_char();
      }
    }

    if (operators.find(op) != operators.end()) {
      current_token = std::make_shared<OperatorToken>(line, col, op);
    }

    if (op == "//") {
      while (peek_char() != '\n') advance_char();
      advance();
    }
  }
}

std::shared_ptr<Token> Tokenizer::peek()
{
  return current_token;
}

