--! select_nightmare_domain
SELECT txt, json, nb, arr FROM nightmare_domain;

--! insert_nightmare_domain (composite?)
INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES (:txt, :json, :nb, :arr, :composite);

--! select_nightmare_domain_null: (txt?, json?, nb?, arr?[?], composite?)
SELECT * FROM nightmare_domain;