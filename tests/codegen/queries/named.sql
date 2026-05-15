--: Id()
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

--: named_composite(wow?,such_cool?)
--: "named_composite.with_dot"("this.is.inconceivable"?)

--! new_named_complex NamedComplexParams(named_with_dot?)
INSERT INTO named_complex (named, "named.with_dot") VALUES (:named, :named_with_dot);
--! named_complex: NamedComplex("named.with_dot"?)
SELECT * FROM named_complex;
