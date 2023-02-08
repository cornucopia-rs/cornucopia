--: Nullity(texts[?], composite?)
--: NullityParams(texts[?], composite?)
--: nullity_composite(jsons?[?])

--! new_nullity NullityParams
INSERT INTO nullity(texts, name, composite) VALUES (:texts, :name, :composite); 
--! nullity: Nullity
SELECT * FROM nullity;
