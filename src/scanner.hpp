#pragma once

#include <fstream>
#include <string>
#include <tuple>
#include <sstream>
#include <iostream>
namespace lexer
{
class Scanner
{
    private:
        std::ifstream _arquivo;
        bool _acabou;
        std::string _arquivo_str;
        size_t _pos;
    public:
        Scanner(std::ifstream&& arquivo) : 
        _arquivo(std::move(arquivo)), 
        _acabou(false),
        _arquivo_str(""),
        _pos(0)
        {
            if (!_arquivo)
            {
                throw std::runtime_error("Arquivo foi não pode ser lido");
            }
            std::ostringstream buffer;
            buffer << _arquivo.rdbuf();
            _arquivo_str = buffer.str();
        }
        char get_next();
        char peek_next() const;
        bool get_termino() const
        {
            return _acabou;
        }
};
}