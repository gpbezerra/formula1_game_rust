# Simulador de corrida
Programa para simular uma corrida de Fórmula 1.

## Resumo

O projeto simula uma corrida com base em vários parâmetros, como tamanho da pista, habilidade dos corredores, tipo de pneu, etc. A corrida é simulada por uma lógica executada a cada volta, que funciona como uma espécie de "turno", onde alguns eventos podem acontecer. Ao final da corrida, o programa retorna a tabela de classificação.  
Os dados relativos aos pilotos são carregados de um arquivo json.  

O projeto foi feito para a disciplina IC845 - Tópicos Especiais em Linguagem de
Programação no período de 2022.1, no qual foi abordada a linguagem Rust. Para
isso, foi sugerido que algumas estruturas fossem implementadas, mas dada a
natureza do projeto não encontramos uma forma natural de adicioná-las (por
exemplo, traits).  

Algumas funcionalidades (como a definição de uma corrida *dentro* do programa),
embora não muito complicadas, não foram implementadas por questões de tempo.

## Ideias / Pendências

- [ ] Randomizador de corredores
- [x] Habilidade dos pilotos afetar chance de acidente (aleatória e em ultrapassagem)
- [ ] Adicionar arquivo de configuração para definir _todos_ os parâmetros (taxa de desgaste dos pneus, coeficientes de chances, etc)
- [x] Definir funções de chance de
  - [x] ultrapassagem
  - [x] acidentes *[Thalles]: por hora, apenas acidentes "solo"*
  - [x] pit-stop
- [x] Transformar loop de `next_lap()` em uma função de `race`
- [x] Implementar seleção da troca do pneu ao fazer um pit-stop. *[Thalles]: por hora, deixei aleatório* 
- [x] Implementar safety car

## Corrida

A corrida é representada por uma série de voltas, que são uma sequência de
eventos que podem ou não ocorrer. Uma corrida acaba quando seu número
pré-definido de voltas ocorrer.  

Possíveis eventos que podem ocorrer durante uma corrida:
- A cada volta os pilotos podem realizar um **pit-stop**, perdendo posições mas
  recuperando a condição de seus pneus e podendo trocar o tipo;
- A cada volta os pilotos podem realizar o evento **ultrapassagem**; uma ultrapassagem também implica em uma chance de ocorrer um acidente envolvendo quem ultrapassa além do piloto sendo ultrapassado.  
- A cada volta os pilotos tem um risco de sofrer um acidente; caso o piloto sofra um acidente ele estará fora da corrida, gerando um evento **safety car**. Os pilotos que saírem da corrida também estarão na tabela de classificação com o status "disabled".

## Eventos 

### Ultrapassagem

1. Cada piloto pode fazer apenas uma ultrapassagem por volta;
2. Quando uma ultrapassagem é realizada, o piloto no qual foi ultrapassado é impossibilitado de realizar uma ultrapassagem na mesma volta;
3. As ultrapassagens começam a ocorrer da ultima posição até a primeira, verificando a probabilidade de cada ultrapassagem e acidente durante a ultrapassagem.  

Para ocorrer uma ultrapassagem, são considerados 3 fatores:
- Tipo de pneu
- Desgaste do pneu
- Habilidade do piloto

### Pit-stop

Os pilotos, ao fazer um pit-stop, perdem 3 posições na corrida, mas tem a possibilidade de trocar o tipo de pneu (ou manter o atual). A condição do pneu também é restaurada para 100%.  

A chance de um piloto decidir fazer um pit-stop é definida de acordo com a condição dos pneus, tendendo a ser mais alta conforme a condição fica muito baixa. A chance é zero até que a condição dos pneus alcance o parâmentro `pit_stop_threshold`, definido pela configuração da corrida.  

### Safety Car

**Regras:**

- Não poderão ser feitas ultrapassagens nas próximas 2 voltas;
- Pit-stops serão permitidos, e pilotos perdem apenas uma posição caso parem;
- O desgaste dos pneus a cada volta permanece normal.

## Atributos dos pilotos

### Tipos de Pneu 

Existem 3 tipos de pneus: Duro, Médio e Macio.  
Assim como na Formula 1, os pneus macios são mais rápidos por conta da aderência e por terem uma maior facilidade em chegar na temperatura ideal, porém, desgastam de forma mais rápida.  
Os pneus médios, como o nome sugerem, são intermediários: mais velozes que pneus duros e menos velozes que os macios, respectivamente mais resilientes que pneus macios e menos resilientes que pneus duros.  
Pneus duros são os mais resilientes, porém, com menor velocidade.  
Com essas possibilidades de trocas de pneus, podemos pensar nas mais diversas estratégias durante uma corrida, trazendo uma complexidade bem interessante.  

### Desgaste dos pneus 

Usaremos um valor percentual para representar o desgaste dos pneus de cada piloto durante uma corrida. Esse valor irá influenciar na chance de ultrapassagem e também irá determinar a chance do piloto fazer um pit-stop. Esse valor será atualizado a cada volta.

A cada ultrapassagem realizada existe uma probabilidade de ocorrer um acidente. **Nesse caso, o acidente irá retirar os dois pilotos envolvidos na ultrapassgem da corrida**, gerando um evento **Safety Car**.
