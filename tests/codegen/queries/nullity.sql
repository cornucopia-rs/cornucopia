--: Nullity(texts[?], composite?)
--: NullityParams(texts[?], composite?)
--: TestNestedComposite(composite[].jsons?, composite[].id?)
--: TestDirectComposite(composite?.jsons?, composite?.id?)
--: nullity_composite(jsons?[?])

--! new_nullity NullityParams
INSERT INTO nullity(texts, name, composite) VALUES (:texts, :name, :composite);

--! nullity: Nullity
SELECT * FROM nullity;

--! test_nested_nullity: (composite[].jsons?, composite[].id?)
SELECT composite FROM nullity WHERE composite IS NOT NULL;

--! test_single_nested: (composite[].jsons?)
SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;

--! test_nested_array: (composite[].jsons?, composite[].id?)
SELECT ARRAY[composite, composite] as composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;

--! test_named_nested: TestNestedComposite
SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;

--! test_direct_nullity: (composite?.jsons?, composite?.id?)
SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;

--! test_single_direct: (composite?.jsons?)
SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;

--! test_named_direct: TestDirectComposite
SELECT composite FROM nullity WHERE composite IS NOT NULL LIMIT 1;
