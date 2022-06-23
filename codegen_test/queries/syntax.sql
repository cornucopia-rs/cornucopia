
--:CompactRow()
     --:          SpaceRow     ()
--:CompactField(a?,b?,c?)
--: SpaceField      (  a?   ,  b?  ,  c?  )   

--simple comment

--! select_compact
SELECT * FROM clone;
      --!      select_spaced   
      SELECT * FROM clone ;   

  --        spaced comment

--!implicit_compact(name?,price?):(id?)
INSERT INTO named (name, price, show) VALUES (:name, :price, false) RETURNING id;
             --!  implicit_spaced        (     name? , price? ) :       ( id? ) 
INSERT INTO named (name, price, show) VALUES (:name, :price, false) RETURNING id;

-- Multi line
-- Comment

--!named_compact Params():Row()
INSERT INTO named (name, price, show) VALUES (:name, :price, false) RETURNING id;
      --!       named_spaced            ParamsSpace  ()     :        RowSpace  () 
INSERT INTO named (name, price, show) VALUES (:name, :price, false) RETURNING id;

--! tricky_sql
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a bind_param\', :price);
--! tricky_sql1
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a :bind_param', :price);
--! tricky_sql2
INSERT INTO syntax ("trick:y", price) VALUES ('this is not a '':bind_param''', :price);
--! tricky_sql3
INSERT INTO syntax ("trick:y", price)  VALUES ($$this is not a :bind_param$$, :price);
--! tricky_sql4
INSERT INTO syntax ("trick:y", price) VALUES ($tag$this is not a :bind_param$tag$, :price);
--! tricky_sql6
INSERT INTO syntax ("trick:y", price) VALUES (e'this is not a '':bind_param''', :price);
--! tricky_sql7
INSERT INTO syntax ("trick:y", price) VALUES (E'this is not a \':bind_param\'', :price);
--! tricky_sql8
INSERT INTO syntax ("trick:y", price) VALUES (e'this is ''not'' a \':bind_param\'', :price);
--! tricky_sql9
INSERT INTO syntax ("trick:y", price) VALUES (E'this is \'not\' a \':bind_param\'', :price);

--! syntax
SELECT * FROM syntax;


-- Multi

-- Comment