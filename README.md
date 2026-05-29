# Atividade da Semana - Reescrita de Algoritmos em Rust
Disciplina: Estruturas de Dados e Análise de Algoritmos  
Professor: Alexandre Montanha de Oliveira  

## Análise de Complexidade Assintótica (Big-O)

## Exercício 1: Verificar Primeiro
**Complexidade:** $O(1)$  
**Lógica do algoritmo:**  
O algoritmo apenas verifica se a lista está vazia. Caso não esteja, retorna diretamente o primeiro elemento (índice 0), ignorando o restante da estrutura.  
**Justificativa da complexidade:**  
Não existem laços de repetição (loops) ou chamadas recursivas. O tempo de execução é constante e independente do tamanho $n$ da entrada, pois o acesso a um índice específico de um vetor leva tempo computacional imediato.

## Exercício 2: Somar Lista
**Complexidade:** $O(n)$  
**Lógica do algoritmo:**  
Percorre sequencialmente cada elemento presente na lista, acumulando seus valores em uma variável mutável de totalização.  
**Justificativa da complexidade:**  
Existe exatamente 1 loop simples que itera linearmente sobre toda a lista. O número de operações cresce de forma perfeitamente proporcional ao tamanho $n$ da entrada.

## Exercício 3: Busca Binária
**Complexidade:** $O(\log n)$  
**Lógica do algoritmo:**  
Realiza uma busca em uma lista previamente ordenada dividindo o intervalo de busca pela metade a cada iteração, descartando a metade onde o alvo certamente não está.  
**Justificativa da complexidade:**  
O espaço de busca é reduzido pela metade a cada passo do laço `while` ($n \rightarrow n/2 \rightarrow n/4 \rightarrow \dots \rightarrow 1$). O número de iterações necessárias no pior caso é o logaritmo na base 2 de $n$.

## Exercício 4: Pares com Soma
**Complexidade:** $O(n^2)$  
**Lógica do algoritmo:**  
Compara cada elemento da lista com todos os elementos subsequentes para verificar quais combinações de pares somam exatamente o valor do alvo.  
**Justificativa da complexidade:**  
O algoritmo utiliza dois loops aninhados. O loop externo roda $n$ vezes e o interno roda em média $n/2$ vezes. Multiplicando as iterações, temos uma relação quadrática de crescimento de tempo.

## Exercício 5: Imprimir Pares e Pares
**Complexidade:** $O(n^2)$  
**Lógica do algoritmo:**  
O algoritmo executa duas tarefas sequenciais: primeiro imprime cada elemento individualmente e, depois, gera e imprime todas as combinações de pares possíveis da lista.  
**Justificativa da complexidade:**  
Apresenta dois blocos isolados. O Bloco 1 possui complexidade linear $O(n)$. O Bloco 2 possui dois loops aninhados que cobrem toda a lista, gerando $O(n^2)$. Pela regra da soma da notação assintótica, retém-se apenas o termo de maior ordem: $O(n + n^2) = O(n^2)$.

## Exercício 6: Potências de Dois
**Complexidade:** $O(\log n)$  
**Lógica do algoritmo:**  
Imprime os valores de potências de 2 sequencialmente enquanto o multiplicador atual for menor que o número limite $n$ fornecido.  
**Justificativa da complexidade:**  
A variável de controle do laço `while` dobra de valor a cada iteração (`i *= 2`). Isso faz com que o algoritmo alcance o limite $n$ de forma exponencialmente rápida, executando um número logarítmico de passos em relação a $n$.

## Exercício 7: Fibonacci Recursivo
**Complexidade:** $O(2^n)$  
**Lógica do algoritmo:**  
Calcula o enésimo termo da sequência de Fibonacci de forma recursiva, quebrando o problema na soma das duas chamadas recursivas anteriores até atingir os casos base.  
**Justificativa da complexidade:**  
Cada chamada da função ramifica-se em duas novas chamadas em quase todos os níveis da árvore de recursão. Sem técnicas de memorização, esse comportamento gera um crescimento exponencial de processamento.

## Exercício 8: Ordenação Bolha (Bubble Sort)
**Complexidade:** $O(n^2)$  
**Lógica do algoritmo:**  
Percorre repetidamente a lista, comparando elementos adjacentes e trocando-os de lugar caso estejam na ordem incorreta, empurrando os maiores elementos para o final.  
**Justificativa da complexidade:**  
Utiliza dois laços de repetição aninhados. Mesmo com a sutil redução do limite do loop interno a cada iteração (`n - i - 1`), o comportamento do pior caso (lista inversamente ordenada) exige $O(n^2)$ comparações e trocas.

## Exercício 9: Produto de Matrizes
**Complexidade:** $O(n^3)$  
**Lógica do algoritmo:**  
Realiza a multiplicação algébrica clássica entre duas matrizes quadradas de dimensão $n \times n$, computando a soma dos produtos de linhas por colunas.  
**Justificativa da complexidade:**  
Possui três laços de repetição profundamente aninhados (controlados pelas variáveis `i`, `j` e `k`), cada um iterando rigorosamente de $0$ até $n$. O número total de operações elementares é exatamente $n \times n \times n = n^3$.

## Exercício 10: Merge Sort
**Complexidade:** $O(n \log n)$  
**Lógica do algoritmo:**  
Algoritmo de ordenação baseado em divisão e conquista. Divide recursivamente a lista ao meio até que existam sublistas unitárias e depois combina-as de forma ordenada (etapa de merge).  
**Justificativa da complexidade:**  
A divisão recursiva divide o problema de forma logarítmica, gerando uma árvore de altura $O(\log n)$. Em cada nível dessa árvore, a operação de intercalação (fusão) dos elementos processa linearmente as sublistas, gastando $O(n)$. Multiplicando a altura pelo trabalho por nível, obtém-se $O(n \log n)$.

