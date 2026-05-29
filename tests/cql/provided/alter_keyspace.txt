ALTER KEYSPACE cycling
WITH REPLICATION = {
    'class' : 'SimpleStrategy',
    'replication_factor' : 2,
};

ALTER KEYSPACE food_cql
WITH REPLICATION = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
} AND graph_engine = 'Core';
