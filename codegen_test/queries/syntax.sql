
--:row CompactRow()
     --:     row     SpaceRow     ()
--:param CompactField(a?,b?,c?)
--:param SpaceField      (  a?   ,  b?  ,  c?  )   

--simple comment

--! select_compact
SELECT * FROM clone;
      --!      select_spaced   
      SELECT * FROM clone ;   

  --        spaced comment

--!implicit_compact(name?,price?):(id?)
INSERT INTO item (name, price, show) VALUES (:name, :price, false) RETURNING id;
             --!  implicit_spaced        (     name? , price? ) :       ( id? ) 
INSERT INTO item (name, price, show) VALUES (:name, :price, false) RETURNING id;

-- Multi line
-- Comment

--!named_compact Params:Row
INSERT INTO item (name, price, show) VALUES (:name, :price, false) RETURNING id;
      --!       named_spaced            Params     :        Row  
INSERT INTO item (name, price, show) VALUES (:name, :price, false) RETURNING id;

--! tricky_sql
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a bind_param', :price);
--! tricky_sql1
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a :bind_param', :price);
--! tricky_sql2
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a '':bind_param''', :price);
----! tricky_sql3
--INSERT INTO item (name, price, show) VALUES ($$this is not a :bind_param$$, :price, true);
----! tricky_sql4
--INSERT INTO item (name, price, show) VALUES ($:tag$this is not a :bind_param$:tag$, :price, true);
--! tricky_sql6
INSERT INTO syntax ("trick:y", price) VALUES (e'this is not a '':bind_param''', :price);
----! tricky_sql7
--INSERT INTO item (name, price, show) VALUES (E'this is not a \':bind_param\'', :price, true);

--! syntax
SELECT * FROM syntax;


-- Multi

-- Comment