CREATE DOMAIN domain_txt AS TEXT;
CREATE DOMAIN domain_json AS JSON;
CREATE DOMAIN domain_nb AS INT;
CREATE DOMAIN domain_array AS domain_json[];

CREATE TABLE nightmare_domain (
    txt domain_txt,
    json domain_json,
    nb domain_nb,
    arr domain_array
);