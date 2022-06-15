--: Named(price?)
--: NamedParams(price?)

--! new_named_visible NamedParams: Id
INSERT INTO named (name, price, show) VALUES (:name, :price, true) RETURNING id ; 
--! new_named_hidden NamedParams: Id
INSERT INTO named (price, name, show) VALUES (:price, :name, false) RETURNING id;
--! named: Named
SELECT * FROM named;
--! named_by_id: Named
SELECT * FROM named WHERE id = :id;

--: NamedComplexParams()
--: named_composite(wow?,such_cool?)

--! new_named_complex NamedComplexParams
INSERT INTO named_complex (named) VALUES (:named);
--! named_complex
SELECT * FROM named_complex;
