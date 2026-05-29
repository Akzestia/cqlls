CREATE SEARCH INDEX IF NOT EXISTS
ON wiki.solr
WITH COLUMNS id, body {excluded : false};
    
CREATE SEARCH INDEX
ON wiki.solr
WITH CONFIG { realtime:true }
AND OPTIONS { reindex : false };
    
CREATE SEARCH INDEX IF NOT EXISTS
ON wiki.solr
WITH COLUMNS c1, c2 {docValues:true}
    AND PROFILES spaceSavingAll
AND CONFIG {directoryFactory:'encrypted'};
    
CREATE SEARCH INDEX
ON wiki.solr
WITH COLUMNS * { docValues:true };
    
CREATE SEARCH INDEX
ON wiki.solr
WITH COLUMNS field1 { indexed:true }, field2 { indexed:false };
    
CREATE SEARCH INDEX
ON wiki.solr
WITH COLUMNS tuplefield.field1 {docValues:true};
    
CREATE SEARCH INDEX
ON wiki.solr
WITH COLUMNS field1 { excluded:true }, field2 { excluded:false };

CREATE SEARCH INDEX
ON wiki.solr
WITH PROFILES spaceSavingNoJoin;
