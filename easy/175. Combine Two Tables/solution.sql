SELECT e.firstName, e.lastName, s.city, s.state
FROM Person e
LEFT JOIN Address s
ON e.personID=s.personID;