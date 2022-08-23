# Simulador de corrida
Programa para simular uma corrida de formula 1.

## Corrida

Objetivo: Gerar uma corrida com número x de pilotos e um número y de voltas, ao final a corrida deve retornar a tabela de classificação, dando ênfanse no podium. Os dados relativo aos pilotos serão consumidos através de um arquivo json. As posições de largada dos pilotos na corrida serão ordenadas de maneira aleatória. 

Possíveis eventos que podem ocorer durante uma corrida:
- A cada volta será avaliado se evento: **pit-stop** pode ocorrer.
- A cada volta os pilotos podem realizar o evento:**ultrapassagem**.
- A cada volta os pilotos tem 1% de chance de sofrer um acidente, caso o piloto sofra um acidente ele estará fora da corrida, gerando um evento:**safety car**. *Observação: o pilotos que sairem da corrida também estarão na tabela de classificação com o status "out"*.

A cada volta o valor de "desgaste do pneu" de cada piloto deve ser alterado de acordo com seu tipo.

_________________

## Ultrapassagem

**Regras**:

1. Cada piloto pode fazer apenas uma ultrapassagem por volta;
2. Quando uma ultrapassagem é realizada, o piloto no qual foi ultrapassado é impossibilitado de realizar uma ultrapassagem na mesma volta;
3. As ultrapassagens começam a ocorrer da ultima posição até a primeira, verificando a probabilidade de cada ultrapassagem e acidente durante a ultrapassagem

Para ocorrer uma ultrapassagem são considerados 3 fatores
- Tipo de pneu
- Desgaste do pneu
- Habilidade do piloto

#### | Calcular a porcentagem da ultrapassagem ocorrer baseada nos tipos de pneu |

Existem 3 tipos de pneus: Duro, Médio e Macio. Assim como na formula 1, os pneus macios são mais rápidos por conta da aderência e por terem uma maior facilidade em chegar na temperatura ideal, porém, desgatam de forma mais abrupta. Os pneus médios passam a ideia de médio mesmo, mais velozes que pneus duros e menos velozes que os macios, respectivamente mais resilientes que pneus macios e menos resilientes que pneus duros. Pneus duros são os mais resilientes porém com menor velocidade. Com essas possibilidades de trocas de pneus podemos pensar nas mais diversas estratégias durante uma corrida, trazendo uma complexidade bem interessante.

*Legenda: pneu piloto atual -> pneu piloto acima (piloto a ser ultrapassado). O símbolo " -> " representa a chance do primeiro pneu ultrapassar o segundo.*

>##### **Pneu Macio**
>Pneu Macio -> Pneu Médio = 70% de chance de ultrapassagem
>Pneu Macio -> Pneu Duro = 90% de chance de ultrapassagem

>##### **Pneu Médio**
>Pneu Médio -> Pneu Macio = 30% de chance de ultrapassagem
>Pneu Médio -> Pneu Duro = 60% de chance de ultrapassagem

>##### **Pneu Duro**
>Pneu Duro -> Pneu Macio = 10%  de chance de ultrapassagem
>Pneu Duro -> Pneu Médio = 40% de chance de ultrapassagem

#### | Cálcular desgaste dos pneus |

Usaremos um valor de 0 a 10 para representar o desgaste do pneu de um piloto durante uma corrida. Esse número irá influenciar na porcentagem da ultrapassagem e também irá determinar a porcentagem de um pit-stop ocorrer. Esse valor será atualizado a cada nova volta iniciada, podemos observar o valores de desgate dos pneus por volta abaixo: 

>##### **Valores dos desagaste do pneu por volta** 
> - Pneu duro = + 0,3  
> - Pneu medio = + 0,5
> - Pneu macio = + 0,7

A cada 1 unidade do valor "desgaste de pneu" a chance de ultrapassagem diminui em 10% relativo a porcentagem atual. Exemplo: a probabilidade da ultrapassagem atual é de 40% e o desgaste do pneu é 2 unidades (20%), nesse caso será retirado 20% de 40% da probabilidade da ultrapassagem ocorrer. ***(não sei se ficou muito claro)***

#### | Calcular habilidade |

*Legenda: PH = Porcentagem baseada na habilidade |  PT = Piloto ultrapassante | PD = Piloto ultrapassado*

Para medir a habilidade de cada piloto estamos atribuindo um valor de 0 a 100. Dado esse valor, pegamos a habilidade dos dois pilotos envolvidos no evento de ultrapassagem (PT e PU) e subtraimos um do outro, segue a fórmula abaixo:

> **PH = PT - PD**

O valor de PH será aplicado diretamente no valor da porcentagem de ultrapassagem atual. Note que o valor o valor de PH pode ser negativo, diminuindo assim a probabilade do PT ultrapassar um PD que possui maior habilidade que ele, caso seja positivo o valor é acrescentando a porcentagem de ultrapassagem.

A cada ultrapassagem realizada existe uma probabilidade de 5% de ocorrer um acidente. **Nesse caso o acidente irá retirar os dois pilotos envolvidos na ultrapassgem da corridda**, gerando um evento:**safety car**

_________________

## Pit-stop

O piloto assim que faz um pit-stop perde 3 posições na corrida. O piloto irá continuar com o mesmo tipo de pneu por toda a corrida, baseado em como foi definido anteriormente o tipo de pneu no arquivo json, nesse caso iremos deixar pneu fixo apenas por questões de simplificação.

>##### **Chance dos pit-stop ocorrer**
> Valor desgaste do pneu == 7 -> 20% de chance de ocorrer um pit-stop
> Valor desgaste do pneu == 8 -> 40% de chance de ocorrer um pit-stop
> Valor desgaste do pneu == 9 -> 70% de chance de ocorrer um pit-stop
> Valor desgaste do pneu == 10 -> 100% de chance de ocorrer um pit-stop

***Pit-stops em safety car ao invés do piloto perder 3 posições como definido a cima, o piloto perde apenas 1 posição***
_________________

## Safety Car

**Regras:**

- Não poderão ser feitas ultrapassagens nas próximas 2 voltas;
- Pit-stops serão permitidos caso estejam na condição de acontecer;
- Pit-stops em safety car o piloto perde apenas 1 posição;
- A contagem de desgaste dos pneus por volta permanece

_________________

## Dados

Tipos de pneu: Duro, Médio, Macio.

Estrutura piloto:
 - nome: string
 - habilidade: numero de 0 a 100
 - tipo de pneu escolhido: enum tipo de pneu
 - desgate do pneu: numero de 0 a 10
 - fazer ultrapassagem: boolean

Observação: Os range dos valores de "habilidade" e "desgaste do pneu" podem ser alteradas caso eu descubra uma lógica melhor de funcionar matematicamente

_________________

## Algumas coisas que ainda não faço ideia de como realizar

- 
- 