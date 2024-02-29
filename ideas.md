**Workers**

Internal pool of threads: 
- fetching set: http gets
- parsing set: extract data from webpages, encode it to the sieve

Points: 
- Fetching threads should be an order of magnitude more than parsing threads.
- Figure out who should query the sieve to get the next url to visit

**Parsing**

- Remove intrapage links such as `url.com#section`


**Sieve**

- Enclose this in a Dispatcher


**Workbench component**

- Include the Sieve
- Implement politeness via a priority queue

**Save to disk**

- Probabilistic save on disk via digest
- Read the google paper on quasi-duplicated pages

