--! select_nightmare_domain
SELECT txt, json, nb, arr FROM nightmare_domain;

--! insert_nightmare_domain
INSERT INTO nightmare_domain (txt, json, nb, arr) VALUES (:txt, :json, :nb, :arr);

--! select_nightmare_domain_null: (txt?, json?, nb?, arr?)
SELECT txt, json, nb, arr FROM nightmare_domain;