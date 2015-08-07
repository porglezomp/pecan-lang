#ifndef PECAN_TOKEN_EOF_TOKEN_HPP_INCLUDED
#define PECAN_TOKEN_EOF_TOKEN_HPP_INCLUDED

#include "token.hpp"

class EOFToken : public Token {
public:
  const TokenType type = EOFTOKEN;
  EOFToken(int line, int col) : Token(line, col) { }
  virtual ~EOFToken() { }
};

#endif
