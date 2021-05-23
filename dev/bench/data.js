window.BENCHMARK_DATA = {
  "lastUpdate": 1621794272485,
  "repoUrl": "https://github.com/vorner/arc-swap",
  "entries": {
    "Track benchmarks": [
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "18cacb53939503210e7598993eef6b87fc8834b2",
          "message": "Keep benchmarks in GH pages",
          "timestamp": "2021-01-02T19:13:52Z",
          "url": "https://github.com/vorner/arc-swap/pull/51/commits/18cacb53939503210e7598993eef6b87fc8834b2"
        },
        "date": 1609689524602,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 17,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 33,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 69,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 121,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 27,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 38,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 99,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 928,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 78,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 140,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 178,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 915,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "10355d69139fa26193de364012d1e5ca8614012d",
          "message": "Merge pull request #51 from vorner/keep-benchmarks\n\nKeep benchmarks in GH pages",
          "timestamp": "2021-01-03T17:27:54+01:00",
          "tree_id": "bf078b66a15959df8a86ab88f8a6b53d81bafa52",
          "url": "https://github.com/vorner/arc-swap/commit/10355d69139fa26193de364012d1e5ca8614012d"
        },
        "date": 1609691667321,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 18,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 105,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 28,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 46,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 101,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 900,
            "range": "± 284",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 102,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 150,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 209,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1085,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64dcf40ad952d87f6f294f9170f272d95bd91f54",
          "message": "Merge pull request #50 from vorner/no-half-lock\n\nReplace the half-lock with helping strategy",
          "timestamp": "2021-01-03T18:01:06+01:00",
          "tree_id": "259ee9c1d6266df99362622b870419e1093cab23",
          "url": "https://github.com/vorner/arc-swap/commit/64dcf40ad952d87f6f294f9170f272d95bd91f54"
        },
        "date": 1609693754486,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 176,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 34,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 56,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 74,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1415,
            "range": "± 1387",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 190,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 227,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1388,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "distinct": true,
          "id": "0426a36d46ed9ea8473b8ea0b7e0250c958eb54a",
          "message": "Don't fail on slow benchmarks\n\nThese are often more noise than a problem, because they are on CI and we\nhave a lot of them.",
          "timestamp": "2021-01-03T18:22:03+01:00",
          "tree_id": "18a506380ae07625886cd64b876c89d300b7905f",
          "url": "https://github.com/vorner/arc-swap/commit/0426a36d46ed9ea8473b8ea0b7e0250c958eb54a"
        },
        "date": 1609694948652,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 176,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 30,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 57,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 79,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1675,
            "range": "± 512",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 162,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 219,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1050,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "ae37017e3319215f5ba99e8e46ce414ba6c228e4",
          "message": "Allow mapping of dyn-access",
          "timestamp": "2021-04-29T06:43:37Z",
          "url": "https://github.com/vorner/arc-swap/pull/53/commits/ae37017e3319215f5ba99e8e46ce414ba6c228e4"
        },
        "date": 1621448861871,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 51,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 141,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 35,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 51,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 100,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 836,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 80,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 127,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 157,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1034,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "fc733c404ba45ba60cd61734db313629a659c67e",
          "message": "Solve some lints",
          "timestamp": "2021-04-29T06:43:37Z",
          "url": "https://github.com/vorner/arc-swap/pull/54/commits/fc733c404ba45ba60cd61734db313629a659c67e"
        },
        "date": 1621449912177,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 126,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 19,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 40,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 51,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1185,
            "range": "± 415",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 86,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 138,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 181,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1092,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6a58a01790ade363dd88dd84f93ca12b1563f739",
          "message": "Merge pull request #54 from vorner/warnings\n\nSolve some lints",
          "timestamp": "2021-05-19T20:54:39+02:00",
          "tree_id": "c664bd5abee00da29bf69e17485d1a9ff4fca44c",
          "url": "https://github.com/vorner/arc-swap/commit/6a58a01790ade363dd88dd84f93ca12b1563f739"
        },
        "date": 1621450960021,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 37,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 156,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 22,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 48,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 73,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 886,
            "range": "± 374",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 102,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 112,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 166,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1020,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "f13cec6fcf1187080c6d7b7eeae5770070ff8bf3",
          "message": "Allow mapping of dyn-access",
          "timestamp": "2021-05-19T18:54:42Z",
          "url": "https://github.com/vorner/arc-swap/pull/53/commits/f13cec6fcf1187080c6d7b7eeae5770070ff8bf3"
        },
        "date": 1621451050676,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 154,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 23,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 52,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 85,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1300,
            "range": "± 638",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 85,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 130,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 166,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1085,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "376beed33d27a63416e1f468a538a71ec1add16f",
          "message": "Merge pull request #53 from vorner/dyn-access\n\nAllow mapping of dyn-access",
          "timestamp": "2021-05-19T21:35:05+02:00",
          "tree_id": "afa685fdf5be83630d46797af0a22c22416f090c",
          "url": "https://github.com/vorner/arc-swap/commit/376beed33d27a63416e1f468a538a71ec1add16f"
        },
        "date": 1621453286563,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 157,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 22,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 46,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 83,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1060,
            "range": "± 457",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 104,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 136,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1069,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "12ed8ac1bd5cb6cc6adf5b97754ac94ef919fa5b",
          "message": "Don't leave threads running in tests",
          "timestamp": "2021-05-20T22:25:32Z",
          "url": "https://github.com/vorner/arc-swap/pull/55/commits/12ed8ac1bd5cb6cc6adf5b97754ac94ef919fa5b"
        },
        "date": 1621794271873,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 18,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 50,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 143,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 31,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 58,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 71,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1341,
            "range": "± 476",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 93,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 148,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1074,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}