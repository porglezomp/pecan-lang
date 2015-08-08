#ifndef PECAN_TOKEN_KEYWORD_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_KEYWORD_TOKEN_HPP_INCLUDED

#include "token.hpp"

class KeywordToken : public Token {
  std::string value;
public:
  virtual std::string show();
  KeywordToken(int line, int col, std::string value)
    : Token(line, col, KEYWORD), value(value) { }

  virtual ~KeywordToken() { }
};

#endif
