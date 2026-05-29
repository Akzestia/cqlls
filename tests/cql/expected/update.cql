UPDATE cycling.cyclist_name            SET firstname = NULL
WHERE id IN (
5b6962dd-3f90-4c93-8f61-eabfa4a803e2,
fb372533-eb95-4bb4-8685-6ef61e994caa
);

UPDATE cycling.cyclist_name
SET
firstname = 'Marianne',
lastname = 'VOS'
WHERE id = 88b8fd18-b1ed-4e96-bf79-4280797cba80;

UPDATE cycling.cyclist_sponsors_expire
USING            TIMESTAMP 200 AND TTL 20000
SET sponsorship += { 'Tag Heuer' }
WHERE cyclist_name = 'PRIETO, Marcela';

UPDATE cycling.popular_count            SET popularity = popularity + 2
WHERE id = 6ab09bec-e68e-48d9-a5f8-97e6fb4c9b47;

UPDATE cycling.cyclist_name
SET
firstname = 'Anna',
lastname = 'VAN DER BREGGEN'
WHERE id = e7cd5752-bc0d-4157-a80f-7523add8dbcd;

INSERT INTO cycling.upcoming_calendar (
  year,
  month,
  events
)
VALUES (
  2015,
  06,
  [ 'Criterium du Dauphine',
  'Tour de Suisse' ]
);

INSERT INTO cycling.upcoming_calendar (
  year,
  month,
  events
)
VALUES (
  2015,
  07,
  [ 'Tour de France' ]
);

UPDATE cycling.upcoming_calendar            SET events = [ 'Tour de France' ] + events
WHERE year = 2015 AND month = 06;

UPDATE cycling.upcoming_calendar            SET events = events + [ 'Tour de France' ]
WHERE year = 2017 AND month = 05;

UPDATE cycling.upcoming_calendar            SET events[2] = 'Tour de France'
WHERE year = 2015 AND month = 06;

UPDATE cycling.upcoming_calendar            SET events = events - [ 'Tour de France' ]
WHERE year = 2015 AND month = 06;

UPDATE cycling.cyclist_career_teams
SET teams = teams + {'Team DSB - Ballast Nedam'}
WHERE id = 5b6962dd-3f90-4c93-8f61-eabfa4a803e2;

UPDATE cycling.cyclist_career_teams
SET teams = teams - {'DSB Bank Nederland bloeit'}
WHERE id = 5b6962dd-3f90-4c93-8f61-eabfa4a803e2;

UPDATE cycling.cyclist_career_teams            SET teams = {}
WHERE id = 5b6962dd-3f90-4c93-8f61-eabfa4a803e2;

  UPDATE cycling.upcoming_calendar            SET description = description + {
  'Criterium du Dauphine' : 'Easy race'
}
WHERE year = 2015 AND month = 06;

UPDATE cycling.upcoming_calendar            SET events[2] = 'Vuelta Ciclista a Venezuela'
WHERE year = 2015 AND month = 06;

UPDATE cycling.upcoming_calendar USING TTL 10000000
SET events[2] = 'Vuelta Ciclista a Venezuela'
WHERE year = 2015 AND month = 06;

  UPDATE cycling.upcoming_calendar            SET description = description + {
  'Criterium du Dauphine' : 'Easy race', 'Tour du Suisse' : 'Hard uphill race'
}
WHERE year = 2015 AND month = 6;

UPDATE cycling.upcoming_calendar
             SET description = {
  'Criterium du Dauphine' : 'Easy race', 'Tour du Suisse' : 'Hard uphill race'
}
WHERE year = 2015 AND month = 6;

UPDATE cycling.cyclist_stats            SET basics.birthday = '2000-12-12'
WHERE id = 220844bf-4860-49d6-9a4b-6b5d3a79cbfb;

UPDATE cycling.cyclist_id            SET id = UUID()
WHERE lastname = 'WELTEN'
AND firstname = 'Bram'
AND age = 18
IF EXISTS;

UPDATE cycling.cyclist_id            SET id = UUID()
WHERE lastname = 'WELTEN'
AND firstname = 'Bram'
AND age = 18
IF id = 18f471bf-f631-4bc4-a9a2-d6f6cf5ea503;
