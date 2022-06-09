
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

-- Multi

-- Comment