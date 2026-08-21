#pragma once

#include <cstdint>
#include <format>
#include <print>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <variant>

namespace lexer
{

struct Id
{
    static constexpr std::string_view nome = "TK_ID";

    std::string valor;
    explicit Id(std::string&& valor)
        : valor(std::move(valor))
    {}

    Id(const Id&) = delete;
    Id& operator=(const Id&) = delete;

    Id(Id&&) noexcept = default;
    Id& operator=(Id&&) noexcept = default;

};

struct Int
{
    static constexpr std::string_view nome = "TK_NUM";
    std::uint64_t numero;
};

struct Soma
{
    static constexpr std::string_view nome = "TK_PLUS";
    static constexpr std::string_view primitiva = "+";
};

struct Subtracao
{
    static constexpr std::string_view nome = "TK_MINUS";
    static constexpr std::string_view primitiva = "-";
};

struct Multiplicacao
{
    static constexpr std::string_view nome = "TK_TIMES";
    static constexpr std::string_view primitiva = "*";
};

struct Divisao
{
    static constexpr std::string_view nome = "TK_DIV";
    static constexpr std::string_view primitiva = "/";
};

struct Atribuicao
{
    static constexpr std::string_view nome = "TK_ASSIGN";
    static constexpr std::string_view primitiva = "=";
};

struct Delimitador
{
    static constexpr std::string_view nome = "TK_SEMI";
    static constexpr std::string_view primitiva = ";";
};

struct Eof
{
    static constexpr std::string_view nome = "TK_EOF";
    static constexpr std::string_view primitiva = "";
};

struct ErroLexico
{
    static constexpr std::string_view nome = "TK_ERRO";
    std::string erro;
    size_t numero_linha;
    explicit ErroLexico(std::string&& erro, size_t numero_linha)
        : erro(std::move(erro)), numero_linha(numero_linha)
    {}
    ErroLexico(const ErroLexico&) = delete;
    ErroLexico& operator=(const ErroLexico&) = delete;

    ErroLexico(ErroLexico&&) noexcept = default;
    ErroLexico& operator=(ErroLexico&&) noexcept = default;
};

struct QuebraLinha { };

using TokenValue = std::variant<
        Id,
        Int,
        Soma,
        Subtracao,
        Multiplicacao,
        Divisao,
        Atribuicao,
        Delimitador,
        Eof,
        ErroLexico,
        QuebraLinha
    >;

class Token
{
    private:
        TokenValue valor;

    public:
        Token(TokenValue valor)
            : valor(std::move(valor)){ }

        const TokenValue& get_token() const
        {
            return valor;
        }
        Token(const Token&) = delete;
        Token& operator=(const Token&) = delete;
            
        Token(Token&&) noexcept = default;
        Token& operator=(Token&&) noexcept = default;
};
}



namespace std
{

template<>
struct formatter<lexer::Token>
{
    constexpr auto parse(format_parse_context& ctx)
    {
        return ctx.begin();
    }

    auto format(
        const lexer::Token& token,
        format_context& ctx
    ) const
    {
        return std::visit(
            [&](const auto& value)
            {
                using T = std::decay_t<decltype(value)>;

                if constexpr (std::is_same_v<T, lexer::Id>)
                {
                    return std::format_to(
                        ctx.out(),
                        "({}, {})",
                        T::nome,
                        value.valor
                    );
                }
                else if constexpr (std::is_same_v<T, lexer::Int>)
                {
                    return std::format_to(
                        ctx.out(),
                        "({}, {})",
                        T::nome,
                        value.numero
                    );
                }
                else if constexpr (std::is_same_v<T, lexer::ErroLexico>)
                {
                    return std::format_to(
                        ctx.out(),
                        "({}, '{}' na linha {})",
                        T::nome,
                        value.erro,
                        value.numero_linha
                    );
                }
                else if constexpr (std::is_same_v<T, lexer::QuebraLinha>)
                {
                    return std::format_to(
                        ctx.out(),
                        "\n"
                    );
                }
                else
                {
                    return std::format_to(
                        ctx.out(),
                        "({}, {})",
                        T::nome,
                        T::primitiva
                    );
                }
            },
            token.get_token()
        );
    }
};
}