# Lexer / Micro C

Este repositório originalmente seria utilizado para o desenvolvimento do lexer do meu trabalho prático da disciplina de Compiladores.

Porém, as especificações do trabalho acabaram seguindo uma direção diferente: em vez de implementar manualmente a pipeline de análise léxica, o trabalho passou a utilizar a ferramenta **Flex** para gerar o analisador a partir de uma especificação léxica.

Embora o valor acadêmico de utilizar uma ferramenta que abstrai justamente uma das partes mais importantes de um compilador seja, na minha opinião, discutível, decidi simplesmente separar os dois objetivos.

O trabalho da universidade será feito utilizando Flex, conforme solicitado.

Este repositório, por outro lado, continuará existindo como um projeto pessoal: uma implementação própria de um compilador para a linguagem **Micro C**, incluindo lexer, parser e, futuramente, geração de código.

Como o código deste repositório não será mais submetido para avaliação acadêmica, também não vejo motivo para continuar preso às escolhas tecnológicas feitas originalmente para a disciplina. A implementação inicial em **C++** será preservada na branch `main-old`, enquanto a branch `main` será reiniciada com uma nova implementação em **Rust**.

A intenção agora não é reproduzir o comportamento de uma ferramenta como Flex, mas implementar e compreender diretamente os componentes que ela normalmente gera ou abstrai.

Em outras palavras:

* `main-old`: implementação original do lexer em C++;
* `main`: nova implementação do compilador em Rust;
* trabalho da disciplina: Flex pode cuidar disso.

Este projeto passa, portanto, a existir exclusivamente pelo seu valor de aprendizado e experimentação — que, convenientemente, era justamente o motivo pelo qual comecei a escrever o lexer à mão em primeiro lugar.
