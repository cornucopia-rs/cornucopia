--: row Item(price?)
--: param ItemParams(price?)

--! new_item_visible ItemParams: Id
INSERT INTO item (name, price, show) VALUES (:name, :price, true) RETURNING id ; 

--! new_item_hidden ItemParams: Id
INSERT INTO item (name, price, show) VALUES (:name, :price, false) RETURNING id;

--! items: Item
SELECT * FROM item;

--! item_by_id: Item
SELECT * FROM item WHERE id = :id;
