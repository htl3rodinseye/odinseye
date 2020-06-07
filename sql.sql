drop table if exists Structure;
drop table if exists Command;
drop table if exists CommandList;
drop table if exists Exercise;

create table Exercise
(
    pk_id           int auto_increment not null primary key,
    name            varchar(255)       not null,
    status          bool default false,
    group_name      varchar(255)       not null,
    root            varchar(255)       not null,
    description     varchar(255)       not null,
    attributes      varchar(255)
);

create table CommandList
(
    pk_id          int auto_increment not null primary key,
    fk_exercise_id int                not null unique,
    foreign key (fk_exercise_id) references Exercise (pk_id)
);

create table Command
(
    pk_id          int auto_increment not null primary key,
    fk_cmd_list_id int                not null,
    command        varchar(255)       not null,
    foreign key (fk_cmd_list_id) references CommandList (pk_id)
);

create table Structure
(
    pk_id int auto_increment not null primary key,
    fk_exercise_id int not null unique,
    structure_string varchar(1023) not null,
    foreign key (fk_exercise_id) references Exercise(pk_id)
);

insert into Exercise values (1, 'Uebung 1 - Wie bekommt "man" Hilfe?', false, 'basic', 'home/odin', 'Schauen Sie sich mit dem Kommando man die Manual Pages zu folgenden Befehlen an: ls, pwd, cat, cp, mv, rm, mkdir, rmdir. Beispiel: >man ls', null);

insert into Exercise values (2, 'Uebung 2 - Ordner auflisten', false, 'basic', 'home/odin', 'Listen Sie mit dem Kommando ls ihr aktuelles Verzeichnis auf.', null);

insert into Exercise values (3, 'Uebung 3 - Command line options', false, 'basic', 'home/odin', 'Verschiedene Befehle haben command line options welche mit einem - (Bindestrich) an dem Kommando hinzugefuegt werden koennen.Geben sie ls -l ein. -l ist die Funktion, mit welcher die Ausgabe im Long Listing Format ausgegeben wird.', null);

insert into Exercise values (4, 'Uebung 4 - Absoluter Pfadname', false, 'basic', 'home/odin', 'Listen Sie mit dem Kommando pwd den absoluten Pfadnamen des aktuellen Verzeichnisses auf. Ihr Nutzerverzeichnis finden Sie unter dem home Verzeichnis.', null);

insert into Exercise values (5, 'Uebung 5 - Standard line output', false, 'basic', 'home/odin', 'Geben sie ihre Zeitzone mit dem Befehl cat /etc/timezone aus dem timezone Datei aus.', null);

insert into Exercise values (6, 'Uebung 6 - Verzeichnisse erstellen 1', false, 'basic', 'home/odin', 'Der Befehl mkdir wird zum Erstellen vor Verzeichnissen genuetzt. Erstellen sie in ihrem jetzigen Verzeichnis das Verzeichnis „dir1“.', null);

insert into Exercise values (7, 'Uebung 7 - Verzeichnisse erstellen 2', false, 'basic', 'home/odin', 'Listen Sie mit dem Kommando pwd den absoluten Pfadnamen des aktuellen Verzeichnisses auf.Ihr Nutzerverzeichnis finden Sie unter dem home Verzeichnis. ', null);

insert into Exercise values (8, 'Uebung 8 - Kopieren', false, 'basic', 'home/odin', 'Der Befehl cp wird verwendet, um Dateien oder Order zu kopieren. Kopieren sie dir21 von der letzten Uebung in dir1.', null);

insert into Exercise values (9, 'Uebung 9 - Verschieben', false, 'basic', 'home/odin', 'Der Befehl mv wird verwendet, um Dateien oder Order zu verschieben. Verschieben sie dir21 von der letzten Uebung in das Verzeichnis /home/odin.', null);

insert into Exercise values (10, 'Uebung 10 - Loeschen von Verzeichnissen und Dateien', false, 'basic', 'home/odin', 'Der Befehl rm wird verwendet, um Dateien oder Order zu loeschen. Doch ist dieser Befehl sehr stark und fuer diese Uebung nicht verwendet.Loesche mit dem Befehl rmdir das Verzeichnis dir12 im /home/odin Verzeichnis.', null);

insert into CommandList values (1, 1);
insert into CommandList values (2, 2);
insert into CommandList values (3, 3);
insert into CommandList values (4, 4);
insert into CommandList values (5, 5);
insert into CommandList values (6, 6);
insert into CommandList values (7, 7);
insert into CommandList values (8, 8);
insert into CommandList values (9, 9);
insert into CommandList values (10, 10);

insert into Command values (1, 2, 'ls');
insert into Command values (2, 3, 'ls -l');
insert into Command values (3, 4, 'pwd');
insert into Command values (4, 5, 'cat /etc/timezone');
insert into Command values (5, 6, 'mkdir dir1');
insert into Command values (6, 7, 'mkdir -p dir2/dir21');
insert into Command values (7, 1, 'man ls');
insert into Command values (8, 1, 'man pwd');
insert into Command values (9, 1, 'man cat');
insert into Command values (10, 1, 'man cp');
insert into Command values (11, 1, 'man mv');
insert into Command values (12, 1, 'man rm');
insert into Command values (13, 1, 'man mkdir');
insert into Command values (14, 1, 'man rmdir');
insert into Command values (15, 8, 'cp dir2/dir21 dir1');
insert into Command values (16, 9, 'mv dir2/dir21 /home/odin');
insert into Command values (17, 10, 'rmdir /home/odin/dir12');

insert into Structure values (1, 1, '{README:"Herrvoragende Arbeit!"}');

select command from Command
join CommandList CL on Command.fk_cmd_list_id = CL.pk_id
join Exercise E on CL.fk_exercise_id = E.pk_id
where E.pk_id = 1;

-- (S.pk_id, S.structure_string), (CL.pk_id),