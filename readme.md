This is terminal application tool intented to organize ideas, thought process, try results in a meaningfull and graphic tree based way.
I made this project to learn Rust and mysql.

DEPENDENCIES
============
cargo # obviously  
docker # necessary to start the mysql database.  
Probably if you have a database already runnig, just change the connection url in the code. It's hardcoded :)  

RUN
===
docker run --name study-mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=studymqsql -d mysql:latest  
cargo run  


TODO

- sa verific daca la mv, parintele destinatie este acelasi ca si acum
- sa pot sterge o intrebare
	- pot sa fac asta adaugand o noua coloana la fiecare nod cu ... parent question
- header cu culori
- list all: 
	- terms
	- models
	- documentations


MySql migration commands
ALTER TABLE mysql.questions ADD COLUMN root_question int NOT NULL AFTER question_text;
