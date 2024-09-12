# snowflake constructor and deconstruct

## explicacion breve
pid es un process id que basicamente es para ver de donde sale la snowflake, puede ser usado para eso, como no, es personalizado
mid es un machine id es el numero de servidor de donde se crea la snowflake, hace que no se hagan dos al mismo tiempo

despues se obtiene el tiempo actual en ms y se pasa a un u64, despues se resta la fecha 01.01.2021 (se puede cambiar)
la secuencia es basicamente usa secuencia, se usa el 0xfff para que sea hasta 4095 bits
para componer la estructura es basicamente |timestamp(41)|mid(5)|pid(5)|seq(12), en total quedan 63 bits, por lo que hay 2^63 de combinaciones, que son 9.223.372.036.854.775.808 de posibilidades, con la timestamp de 41 bits son 2^41, son 2.199.023.255.552ms, que son 69 años y pico, asi que tira para rato

para desconstruir se hace lo contrario a todo lo que explique, fin 