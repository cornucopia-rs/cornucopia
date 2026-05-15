
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
INSERT INTO syntax ("trick:y", async, enum) VALUES ('this is not a bind_param\', :async, :enum);
--! tricky_sql1
INSERT INTO syntax ("trick:y", async, enum) VALUES ('this is not a :bind_param', :async, :enum);
--! tricky_sql2
INSERT INTO syntax ("trick:y", async, enum) VALUES ('this is not a '':bind_param''', :async, :enum);
--! tricky_sql3
INSERT INTO syntax ("trick:y", async, enum)  VALUES ($$this is not a :bind_param$$, :async, :enum);
--! tricky_sql4
INSERT INTO syntax ("trick:y", async, enum) VALUES ($tag$this is not a :bind_param$tag$, :async, :enum);
--! tricky_sql6
INSERT INTO syntax ("trick:y", async, enum) VALUES (e'this is not a '':bind_param''', :async, :enum);
--! tricky_sql7
INSERT INTO syntax ("trick:y", async, enum) VALUES (E'this is not a \':bind_param\'', :async, :enum);
--! tricky_sql8
INSERT INTO syntax ("trick:y", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', :async, :enum);
--! tricky_sql9
INSERT INTO syntax ("trick:y", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', :async, :enum);
--! tricky_sql10
INSERT INTO syntax ("trick:y", async, enum) VALUES ('this is just a cast'::text, :async, :enum);

--! typeof
SELECT * FROM syntax;

-- Multi

-- Comment

--! select_comment
--- Multi line
---
--- Doc string comment
SELECT * FROM syntax;

--! select_inline_comment
SELECT
    -- remove this comment:
    '-- dont remove this\n' as c1,        -- and this:
    $$-- or this$$ as c2,                 -- and this
    E'-- escape string here' as c3,       -- this too
    -- you guess it, this too
    e'-- another escape string' as c4,    -- and this
    $tag$-- dollar quoted here$tag$ as c5 -- finally this
;

--! semicolon_in_string
INSERT INTO syntax ("trick:y", async, enum) VALUES ('a; b', :async, :enum);
--! semicolon_in_dollar_quote
INSERT INTO syntax ("trick:y", async, enum) VALUES ($$a; b$$, :async, :enum);
--! semicolon_in_tagged_dollar_quote
INSERT INTO syntax ("trick:y", async, enum) VALUES ($tag$a; b$tag$, :async, :enum);
--! semicolon_in_escape_string
INSERT INTO syntax ("trick:y", async, enum) VALUES (E'a\; b', :async, :enum);
--! semicolon_in_comment
SELECT * FROM syntax -- ;where
;
