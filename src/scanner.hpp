#pragma once

#include <fstream>
#include <string>
#include <tuple>

namespace lexer
{
class Scanner
{
    private:
        std::ifstream _arquivo;
        bool _acabou;
    public:
        Scanner(std::ifstream&& arquivo) : 
        _arquivo(std::move(arquivo)), 
        _acabou(false)
        {
            if (!_arquivo)
            {
                throw std::runtime_error("Arquivo foi não pode ser lido");
            }
        }

        std::string get_line();

        bool get_termino() const
        {
            return _acabou;
        }
};
}