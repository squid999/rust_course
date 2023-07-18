#!/usr/bin/env python3

import json
import requests


r = requests.get('http://localhost:8111/api/v1/students')
if r.status_code == 200:
    # print(str(r.content))
    print(json.dumps(json.loads(r.content.decode('utf-8')), indent=2))


r = requests.get('http://localhost:8111/api/v1/classes')
if r.status_code == 200:
    print(r.content)
    print(json.dumps(json.loads(r.content.decode('utf-8')),
          indent=2, ensure_ascii=False))

r = requests.get('http://localhost:8111/api/v1/stucls')
if r.status_code == 200:
    # print(str(r.content))
    print(json.dumps(json.loads(r.content.decode('utf-8')), indent=2))
