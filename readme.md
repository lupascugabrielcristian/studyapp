This is terminal application tool intented to organize ideas, thought process, try results in a meaningfull and graphic tree based way.
I made this project to learn Rust and mysql.

DEPENDENCIES
============
cargo # obviously  
docker # necessary to start the mysql database.  
Probably if you have a database already runnig, just change the connection url in the code. It's hardcoded :)  

SETUP  
=====  
sudo snap install rustup --classic  
rustup install stable  

BUILD
=====
cargo build
cargo build --release	# Gasesc binarul in target/release/

RUN
===
docker run --name study-mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=studymqsql -d mysql:latest  
cargo run  


MySql migration commands
ALTER TABLE mysql.questions ADD COLUMN root_question int NOT NULL AFTER question_text;

Bacup database
docker exec study-mysql /usr/bin/mysqldump -u root --password=studymqsql mysql > /tmp/backup.sql

For importing dump files
docker exec -i study-test mysql -uroot -pstudymqsql mysql < /tmp/study_backup.sql
Ref https://stackoverflow.com/questions/65585749/how-to-import-a-mysql-dump-file-into-a-docker-mysql-container
