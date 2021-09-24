# Schackmotor Dokumentation
## Förord ##

Till den som använder detta library: jag ber om ursäkt för den stökiga koden, det var väldigt många regler som jag inte kände till när jag gjorde kodskelettet, vilket ledde till att det blev många if-satser
i Game::player_move() samt ett scan!-macro som kan vara lite svårt att följa då det innehåller många loopar och match statements. Det finns många förbättringar att göra, och jag har säkert missat något corner case i mina tester 
som totalt buggar ur koden, men isåfall är det bara att skapa en issue så fixar jag det.

Jag tror att jag implementerat det mesta. Jag har inte implementerat threefold repetition, men det kan jag göra om det krävs (kanske spara hashes av brädet eller något?). Det finns inte heller någon undo-funktion i spelet, men jag minns inte om det var något krav.


##Game##

struct data du kan komma åt:
board - spelbrädet, en mutable array [[Piece; 8];8], där varje ruta är en pjäs.
score - ej implementerat
turn_white - en bool som avgör vems tur det är. om den är false är det black.
check_mate - bool
tie - en bool som just nu inte gör något. Det finns en claim_draw()-funktion som returnerar en bool, och då kan man ju assigna det värdet till tie om man vill göra något med det i GUI. 
vector_of_the_fallen - en vecdeque som håller döda pjäser. 
 
I princip det enda du behöver importera för att använda motorn är Game-structen. Nedan följer några användbara metoder:

Game::new_game() - skapar upp ett nytt spelobjekt som du sedan kan kalla andra metoder på.

Game::player_move(&mut self, input: String) -> Result<bool, String> - Tanken är att man utgår från denna metod genom hela spelet. Det tar in input som typ "A2A3\n", kollar om det är ett legal move och utför
sedan ändringen om det är valid. Den är just nu skriven att göra lite basic validation på att input följer formatet som "A2A4\n", jag kan ändra detta om det blir svårt att jobba med av nån anledning, jag valde mest detta då det blev enklast från terminalen. Från GUIs perspektiv så får spelaren, även om de inte kan schack alls, gissa
var de får flytta pjäsen, och sedan validerar biblioteket det. Det finns en metod i pieces som producerar en 8x8-array med legal moves från specificerad position, men just nu håller den sig i funktionens scope.
Men det skulle ju lätt gå att returna den om man vill ge info till spelaren vart de kan flytta en vald pjäs typ.

Game::claim_draw(&mut self) -> bool - denna funktion implementerar fifty move rule typ. Finns en vecdeque som håller de 50 tupler (Piece,u8). Piece är pjäsen som flyttades då, och u8 är storleken på
vector_of_the_fallen när förflyttningen skedde. Så funktionen kollar helt enkelt om det finns en pawn i vår vecdeque, samt om storleken på vector_of_the_fallen har samma storlek på index [0] och index [49].
Funktionen returnerar true om kraven för 50 move rule uppfylls.

Game::pawn_promotion(&mut self, input: String, to_piece: String) -> Result<bool,String> - otestad funktion som jag skrev igår
exempelformat på parametrar:
input: "A8"
to_piece: "bb" (black bishop)
Den validerar färgen på pjäsen och att det är en pawn, sen skapar den en ny pjäs på den platsen av typen som specificeras i to_piece. Parametrarna ska inte innehålla "\n"

##Piece##

Detta är annan viktig struct som finns, men den ska inte behöva användas för att nyttja detta library, utan bara om man vill ha lite info om pjäserna typ. 
