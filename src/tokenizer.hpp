#ifndef PECAN_TOKENIZER_HPP_INCLUDED
#define PECAN_TOKENIZER_HPP_INCLUDED

#include <iostream>
#include <memory>
#include <functional>

#include "token/all_tokens.hpp"

class Tokenizer {
  std::istream &input;
  int line = 0, col = 0;
  char current_char = '\n';
  std::shared_ptr<Token> current_token = nullptr;

  void advance_char();
  char peek_char();
  char take_char();
  std::string take_char_while(std::function<bool (char)> predicate);

public:
  void advance();
  std::shared_ptr<Token> peek();

  Tokenizer(std::istream &in) : input(in) {
    advance_char();
    advance();
  }
};

#endif
