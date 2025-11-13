# Proyecto de Compiladores - Otoño 2025

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP  Proyecto de Compiladores  
Dra. Hilda Castillo Zacatelco  
 
Crear un intérprete para un lenguaje de programación que permita describir de forma 
gráfica una red de computadoras. Sólo se podrán utilizar los siguientes componentes:  
concentradores, computadoras, cable coaxial y cable UTP; las topologías a utilizar son : 
estrella la, bus y un híbrido compuesto por estrella y bus , tomando en cuenta las reglas de 
diseño para una tecnología Ethernet.  Cuand o el concentrador tenga entrada para coaxial, la 
entrada siempre estará en el extremo derecho del concentrador.  
En términos generales, la topología de estrella indica que para formarla es necesario un 
concentrador y varias máquinas , cada m áquina se une a un puerto del concentrador 
mediante cable UTP.  
 
 
 
 
 
 
 
 
 
 
 
En el caso de la topología de bus, se utiliza un segmento de cable coaxial y se pegan a él 
varias máquinas . 
 
 
 
 
 
 
 
La estructura en general de un programa fuente será la siguiente:  
 
programa  nombre ; 
define maquinas  
 lista_de_maquinas  ; 
define concentradores  
 lista_de_concentradores ; 
define coaxial  
 lista_coaxial ; 
 
definición_de_modulos  
 
inicio  
    sentencias  
fin. Maquina  
Concentr ador 
Cable UTP  
Cable Coaxial  
Máquina  
Topología física de bus  Topología física 
de estrella  

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP   
Las palabras en cursiva son palabras reservadas.  Pueden o n o definir se alguno de los 
elementos (maquinas, coaxial o concentradores) o todos . 
Donde: nombre  es un identificador (combinación de letras  mayúsculas o minúsculas , y 
dígitos , donde primer carácter debe ser una letra ), y lista_de_maquinas  es una lista de 
identificadores separados por una coma.  
Ejemplo:  
programa primer;  
define maquinas  
A,B,C;  
 
primer  es el nombre del programa y se han definido tres máquinas llamadas A,B y C.  
lista_de_concentradores , permite definir una lista concentrador es y se ocupará la siguiente 
sintaxis:  
nombre=numero_de_puertos.coaxial ; 
 
se podrán definir el número de concentradores que sean necesarios, separando cada uno de 
ellos por una coma. Si el concentrador tiene salida para cable coaxial se indicará colocando 
un punto y el número uno. Por ejemplo: si definimos un concentrador de 4 puertos y una 
salida para cable coaxial se escribirá: hub1=4.1  
El número de puertos de un concentrador que se podrán aceptar son: 4,8 y 16.  
Ejemplo:  
define concentradores  
hUb1=4.1, Hub2=8.1, hub3=4;  
Esta definición indica que se utilizarán 3 concentradores, los primeros dos tiene n salida 
para cable coaxial y son de 4 y 8 puertos respectivamente, y el último tiene 4 puertos pero 
no tiene salida para cable coaxial.  
 
lista_coaxial, define segmentos de cable coaxial utilizando la sintaxis:  
nombre=metros   sin son  varios segmentos, se separa cada definición de segmento por una 
coma.  
 
Ejemplo:  
define coaxial  
seg1=10, seg2=100, seg3=50;  
 
Los módulos tendrán la siguiente estructura:  
modulo  nombre;  
Inicio  
Sentencias  
fin 
 
Las definiciones acerca de máquinas , concentradores y coaxial, s olo serán globales.  
módulos.  Para invocar un módulo, sólo se deberá in vocar con su nombre.  
 

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP   
 
Sentencias  
Las sentencias que podrán ser utilizadas son las siguientes:  
 
coloca(maquina _o_concentrador, coordenada_X, coordenada_Y);  
Coloca  (dibuja ) una máquina  que estará conectada a un concentrador ( únicamente a un 
concentrador, pa ra conectarla a un coaxial se usará maquinaCoaxial ), o un concentrador en 
las coordenadas especificadas, si la máquina o el concentrador no ha sido definido antes, o 
si las coordenadas en X o Y están fuera del rango, se generará un error . coordenada_X  
y coordenada_Y  son números enteros positivos.  
La interpretación será colocar un objeto gráfico en las coordenadas correspondientes.  
 
coloca Coaxial Concentrador (coaxial , concentrador ); 
Permite dibujar una lín ea que representa al coaxial  asociado a un cierto 
concentrador . La línea del segmento de ca ble coaxial siempre se dibujará de izquierda 
a derecha , es decir, de donde está la entrada del c oaxial del con centrador  hacia la derecha , 
tomando en cuenta que c ada 15 pixeles representará un metro de  cable coaxial.   
 
 
coloca Coaxial  (coaxial , coordenada_X, coordenada_Y , direccion ); 
Permite dibujar una lín ea a partir de las coordenadas coordenada_X, 
coordenada_Y , en la dirección indicada  (cuando no este asociado a ningún 
concentrador ), donde direccion  puede ser arriba , abajo , izquierda  o 
derecha . Cada 15 pixeles representará un metro del cable coaxial.  
 
 
uneMaquinaPuerto(maquina, concentrador, puerto);  
Permite unir una máquina a un concentrador en el número de puerto especificado. Tanto 
máquina como concentrador deberán haber sido definidos con anterioridad , en caso 
contrario se generará un error. puerto  es un número entero positivo y habrá error si el 
puerto ya ha sido asignado con anterioridad o no existe.  
 
asignaPuerto(máquina, concentrador);  
Permite asignar algún puerto disponible del concentrador a una máquina . Habrá error si 
no existen puertos disponibles.  
 
maquinaCoaxial(maquina, coaxial , pos_X ); 
Coloca máquina  en el segmento de cable coaxial  a una distancia pos_X del inicio del 
cable. Se generará un error en caso de que , según las reglas , no se permita colocar una 
máquina a esa distancia, o exista alguna máquina  muy cerca, o pos_X sea mayor que el 
tamaño del cable  o maquina  o coaxial  no exist an. 
 
asignaMaquinaCoaxial(maquina, coaxial);  

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP  Coloca una máquina en el segmento de cable coaxial correspondiente respetando las reglas 
de diseño, se generará un error si la operación no se puede llevar a cabo o la máquina o 
coaxial no existen.  
 
escribe(cadena);  
escribe(variable);  
Esta sentencia permite escribir una cadena o el contenido de una variable, regularmente el 
número de puertos de un concentrador, o la longitud de un cable coaxial.  
 
si (condición) inicio  
Sentencias_caso_verdadero  
fin 
sino inicio  
Sentencias_caso_falso  
fin 
 
La sentencia si, permite revisar una condición, si la condición es verdadera aplica 
sentencias_caso_verdadero , en caso contrario aplica 
sentencias_caso_falso . La estructura sino puede aparecer o no, las palabras 
reservadas de inicio  y fin siempre deberán aparecer, sea el bloque de una o más sentencias. 
La condición siempre deberá estar encerrada entre paréntesis. Esta sentencia podrá ser 
ocupada para revisar el número de puertos en un concentrador o si es posible colocar una 
máquina más en un segmento coaxial, o si un puerto esta disponible o no.  
La condición podrá utilizar operadores relacionales (<  - menor que,  > - mayor que,  <= -
menor o igual que ,  >= - mayor o igual que,  <> - diferente,  = igual) y lógicos (&&  - and, ||  -
or,  ! - not) 
La condición más pequeña estará formada por únicamente una proposición  con operador  
relacional  (ningún operador  lógico ) y la más grande se formará utilizando dos expresiones 
relacionales conectadas por un operador lógico.  
 
Ejemplos:  
(X.puertos >=4) && (X.p[1]=0)  
!(X.p[2])  
 
Para obtener información acerca de los puertos que ya han sido ocupado s y acerca del total 
de puertos de que consta un concentrador, cada uno de ellos manejará un registro con los 
siguientes campos:  
Nombre_concentrador  
puertos=numero;  
 p[numero];  
 disponibles=numero;  
            coaxial= nomb reCoaxial o 0 ; 
 presente=0 o 1;  
  
fin; 

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP  Para conocer  el número  de puertos que tiene el concentrador X , se escribirá: X.puertos . 
Además, para con ocer si deter minado puerta esta ocupado o no, se ocupará el arreglo p  
(p[numero] ), donde numero  será el número de puerto s con los que cuen ta el 
concentrador , el primer índice del arreglo p comenzará siempre en 1, y su contenido será 1 
o 0, 1 indicará que el puerto ya ha sido ocupado y 0 que aún est á desocupado. El campo 
disponibles  contendrá  el número de puertos disponibles en el concentrador. Así para 
saber si el puerto 5 del concentrado A esta disponible se escribe:  
 
Si (A.p[5]=0) inicio  
   uneMaquinaPuerto(maqX,A,5);  
fin 
sino inicio  
   asignaPuerto(maqX,A);  
 
En caso contrario se deberá asignar el puerto disponible.  
Si el concentrador tiene una entrada para coaxial el campo coaxial  contendrá el no mbre 
del coaxial asignado, en caso contrario será 0.  
En el caso de  un cable coaxial, el intérprete utilizará un registro  con los siguientes campos:  
Nombre_ coaxial  
longitud =numero;  
completo=1 o 0;  
num=numero;  
maquina [n]; 
pos[n]; 
presente=1 o 0;  
fin; 
 
En el caso del cable coaxial se puede consultar información accediendo  al campo longitud 
para verificar la longitud del segmento del cable, y al campo completo , si el completo tiene 
1 significa que ya no es posible asignar m ás maquinas  al segmento de cable y si es 0 
significa que aún es posible asignar una máquina al segmento de cable coaxial. num 
contiene el número de máquinas asignadas al cable.  
Para cada máquina ya asignada al coaxial se tiene n los siguientes campos: maquina[ n], 
pos[n], donde  n es el número de máquinas conectadas actualmente al coaxial,  
maquina[i]  contiene el nombre de la i -ésima máquina conectada y pos[i]  contiene la 
posición de la máquina i -ésima en el cable coaxial.  
Las maquinas, los concentradores y coaxiales tendrán un campo llamado presente este 
campo tendrá el valor 1 si el objeto fue colocado  (en pantalla ) y 0 caso contrario.  
Las reglas de diseño para asignar máquinas a un cable coaxial son las siguientes:  
La longitud mínima del cable es de 3 m., la máxima es de 500 m.. Cada máquina deberá 
estar separada al menos 3 m.  
//En el caso de concentradores: se podrán conectar hasta 10 concentradores en cascada 
//utilizando puertos o cable coaxial.  
 
Algunos ejemplos de programas fuentes son:  
 

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP  programa ejemplo;  
define maquinas  
     A,B,C, nodo1, n odo2; 
define concentradores  
    uno=4, dos=8;  
define coaxial  
 seg1=5; 
 
modulo primero;  
inicio 
colocaCoaxial(seg1, 23,5 0, derecha ); 
coloca(nodo1,55,55); 
maquinaCoaxial(nodo1, seg1 , 30); 
si (seg1.completo= 0) inicio  
   coloca(nodo2, 50,70);  
   asignaMaquinaCoaxial(nodo2,  seg1); 
   si (seg1.completo=0) inicio  
      coloca(C,100,70);  
      asignaMaquinaCoaxial(C, seg1);  
      si (uno.coaxial=1) && (uno.p[1]=0) inicio  
         uneMaquinaPuerto(seg1,uno,1);  
      fin     
   fin 
fin 
fin 
 
inicio 
  coloca(A,100,20);  
  coloca(B,130,20);  
  coloca(dos, 120,30);  
  uneMaquinaPuerto(A,dos, 3);  
  uneMaquinaPuerto(B,dos,7);  
  si (uno.presente=1) inicio  
     uneMaquinaPuerto(uno,dos,5);  
  fin 
  primero;  
fin. 
 
 
 
programa ejemplo2;  
define maquinas  
   x23, y34;  
define segmento  
   Coax1=10;  
inicio 
  coloca(x23,30,50);  

PROYECTO DE COMPILADORES                        OTOÑO 2025   
Dra. Hilda Castillo Zacatelco   FCC -BUAP    coloca(y34,30,100);  
  colocaCoaxial(Coax1,40,50 , abajo); 
  maquinaCoaxial( x23,Coax1,1);  
  maquinaCoaxial(y34,Coax1,10);  
fin 
 