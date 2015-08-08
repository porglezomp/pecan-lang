// Copyright 2015 Caleb Jones

#include "lexer.hpp"

// #include <functional>
// #include <string>
// #include <memory>
#include <sstream>
#include <cctype>
#include <set>

#include "token/all_tokens.hpp"

std::set<std::string> keywords = {
  "if", "else", "while", "for", "switch", "case",
  "default", "return", "break", "continue",
  "function", "let", "var",
};

std::set<char> operator_characters = {
  '+', '-', '*', '/', '=', '<', '>', '!', '%', '$',
  '?', ':', '#', '@', '&', '|', '^', '\\', '.'
};

char Lexer::peek_char() { return current_char; }
std::shared_ptr<Token> Lexer::peek() { return current_token; }

char Lexer::take_char()
{
  char tmp = peek_char();
  advance_char();
  return tmp;
}

std::string Lexer::take_char_while(std::function<bool (char)> predicate)
{
  std::string result = "";
  while (predicate(peek_char()) && !input.eof()) {
    result += take_char();
  }
  return result;
}

void Lexer::advance_char()
{
  if (input.eof()) return;
  if (current_char == '\n') {
    line += 1;
    col = 0;
  }
  col += 1;
  current_char = input.get();
}

void Lexer::advance()
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

  if (isalpha(peek_char()) || peek_char() == '_') {  // [_a-zA-Z][_a-zA-Z0-9]*
    std::string ident = take_char_while([] (char c) {
	return isalnum(c) || c == '_';
      });

    if (keywords.find(ident) != keywords.end()) {
      current_token = std::make_shared<KeywordToken>(line, col, ident);
    } else {
      current_token = std::make_shared<IdentToken>(line, col, ident);
    }
  } else if (isdigit(peek_char())) {  // [0-9]+(. [0-9]*)?
    std::string num = take_char_while(isdigit);
    if (peek_char() == '.') {
      num += take_char();
      num += take_char_while(isdigit);
    }

    current_token = std::make_shared<NumToken>(line, col, num);
  } else {
    auto is_operator_character = [] (char c) {
	return operator_characters.find(c) != operator_characters.end(); };
    std::string op = take_char_while(is_operator_character);

    if (op != "") {
      current_token = std::make_shared<OperatorToken>(line, col, op);
      if (op == "//") {
	while (peek_char() != '\n') { advance_char(); }  // consume the line
	advance();  // Get the next real token
      }
    } else {
      current_token = std::make_shared<CharToken>(line, col, peek_char());
      advance_char();
    }
  }
}

