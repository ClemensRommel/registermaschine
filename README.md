# registermaschine
Ein Simulator für eine Registermaschine, die einen Akkumulator besitzt sowie einen Hauptspeicher, in dem sowohl das Program als auch die Daten gespeichert sind. Das Program besteht aus einem Compiler, der die Operationen und Daten in ein Program übersetzt, das danach durch einen Interpreter ausgeführt wird. Der Compiler unterstützt dazu verschiedene Features, wie z.B. labels.

# Programaufbau
Ein Program besteht aus einer Reihe von Operationen, die in die Entsprechenden Codes übersetzt wird, und Zahlen. Dabei wird nicht geprüft, ob eine Operation genügend Operanden hat, oder ob diese Zahlen sind. 

# Beispielprogram - Hello World!
~~~
JUMP start
~~~
Da Programme vom Anfang an ausgeführt werden, werden einige Datendeklarationen übersprungen und zum Start gesprungen. Dies geht mit der Operation JUMP, die zum Operanden springt, in dem Fall ein Label, das noch Später deklariert wird.
~~~
text:
  "Hello World!" ; Auch Strings können verwendet werden. Man könnte auch genauso gut die Zahlen für die Entsprechenden Zeichen-Codes verwenden
  ; Kommentare beginnen übrigens mit ;
  
variablen:
  pointer: text ; Ein verweis auf den Speicherort des Strings
  length: 12 ; Länge des Strings
~~~
"Variablen" funktionieren mit labels: Diese werden im Resultierenden Program einfach durch Die Position, an der sie definiert wurden, ersetzt
~~~
start: ; Beginn des Programms!
  ; Der Text wird durch eine Schleife ausgegeben. Diese beginnt hier
  LOAD pointer  ; Speichere die Referenz auf die derzeitige Position im Akkumulator
  LOADIND ; Lädt die Daten an dem Ort, auf dem im Akkumulator eine Referenz gespeichert ist
  PRINTC ; Druckt den derzeitigen Akkumulatorwert als Buchstaben aus
    ; Also das Zeichen, bei dem die Schleife gerade angekommen ist
  LOAD pointer      ; Lädt die Referenz auf die Derzeitige Position
  ADDI 1            ; Erhöht sie um 1, d.h. auf das nächste Zeichen 
  STORE pointer     ; Die Referenz wird gespeichert.
  SUBSTRACTI text   ; Die Referenz zum Anfang wird abgezogen. Im Akkumulator ist der Abstand zum Anfang gespeichert.
  GREATER length    ; Der Abstand wird mit der länge verglichen. Im Akkumulator ist 0, wenn length größer als der Akkumulator ist
  JUMPIFNZERO start ; Es wird wieder zum Anfang gesprungen, wenn der Abstand <= länge ist
  
  HALT              ; Ansonsten wird das Programm beendet
~~~
# Unterstützte Operationen
Viele Befehle haben eine Variante mit I am Ende. Wo normale Befehle Werte in Speicherzellen verwenden, verwenden diese direkt ihre Operanden

`NOOP` Macht nichts

## Instruktionen zum Arbeiten mit dem Speicher
`LOADI`, `LOAD` Lädt einen Wert aus einer Speicherzelle bzw. direkt in den Akkumulator

`LOADIND` Lädt den Wert der Speicherzelle, auf die eine Referenz im Akkumulator gespeichert ist

`STORE` Speichert den Akkumulator in einer Zelle

`STOREIND` Speichert den Akkumulator an der Stelle, die in einer Zelle gespeichert ist

`MOVE`, `MOVEI` Kopiert die Daten von einer Zelle zur anderen

`MOVEIND` Kopiert die Daten vom Argument zur Zelle gespeichert im Akkumulator

## Instruktionen zum Rechen
`ADDI`, `ADD` Addiert einen Wert aus einer Zelle zum Akkumulator

`SUBTRACTI`, `SUBTRACT` Zieht einen Wert aus einer Zelle vom Akkumulator ab

`MULTIPLYI`, `MULTIPLY` Multipliziert den Akkumulator mit dem Wert in einer Zelle

`DIVIDEI`, `DIVIDE` Dividiert den Akkumulator durch einen Wert in einer Zelle

`REMAINDERI`, `REMAINDER` Berechnet den Rest des Akkumulators und einer Zelle

## Instruktionen zum Arbeiten mit Bits
`SHIFTL`, `SHIFTLI`, `SHIFTR`, `SHIFTRI` Verschiebt die Bits im Akkumulator nach rechts bzw. links

`AND`, `ANDI` Berechnet das Binäre Und mit dem Akkumulator

`OR`, `ORI` Berechnet das Binäre Oder mit dem Akkumulator

`XOR`, `XORI` Berechned das Binäre xor mit dem Akkumulator

`NOT` Kehrt die Bits des Akkumulators um

## Vergleich-Instruktionen
`EQUAL`, `EQUALI` Setzt den Akkumulator zu 1, wenn die Zelle und der Akkumulator gleich sind, sonst 0

`GREATER`, `GREATERI` Setzt den Akkumulator zu 1 wenn die Zelle größer als der Akkumulator ist, sonst 0

`LESS`, `LESSI` Setzt den Akkumulator zu 1 wenn die Zelle kleiner als der Akkumulator ist, sonst 0

## Instruktionen für den Kontrollfluss
`JUMP` Springt zum Parameter

`JUMPIFZERO` Springt zum Parameter, wenn der Akkumulator 0 ist, sonst wird das Programm normal weiter ausgeführt

`JUMPIFNZERO` Springt zum Parameter, wenn der Akkumulator nicht 0 ist, sonst wird das Programm normal weiter ausgeführt

`JUMPLT` Springt zum Parameter, wenn der Akkumulator < 0 ist

`JUMPGT` Springt zum Parameter, wenn der Akkumulator > 0 ist

`CJUMP` Springt zur Stelle, die im Akkumulator gespeichert ist
