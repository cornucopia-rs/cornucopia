--: Character(release_date?)

--! characters : Character
SELECT
    *
FROM
    characters;

--! select_character_by_element
SELECT
    name, element, quality
FROM
   characters
WHERE
    element = :element;
