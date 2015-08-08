#include "char_token.hpp"

std::string CharToken::show()
{
  std::string show = "Char(";
  show += value;
  return show + ")";
}
