CREATE OR REPLACE FUNCTION cycling.left (column text)
    RETURNS NULL ON NULL INPUT
    RETURNS text
    LANGUAGE javascript AS
$$ column.substring(0, num) $$;

CREATE OR REPLACE FUNCTION cycling.flog (input double)
    CALLED ON NULL INPUT
    RETURNS double
    LANGUAGE java AS
$$ return Double.valueOf(Math.log(input.doubleValue())); $$;
