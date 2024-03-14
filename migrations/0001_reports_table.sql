create table scans (
   title varchar not null,
   author varchar not null,
   isbn varchar not null
);

create unique index boox_isbn_idx on scans (isbn);