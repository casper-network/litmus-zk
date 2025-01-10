# litmus-zk
Experimental ZK light client framework for Casper blockchain

# WORK IN PROGRESS

This is very much WIP ... do not even attempt to utilise for production purposes.

# Contents

`/crates`

Set of pure rust crates encapsulating litmus core business logic plus kernel.

`/resources`

Static resources used primarily in testing & prototyping modes.

`/pgms`

Programs to be run within ZK virtual machines ... one program per supported ZK-VM type.  They are designed to be as 'thin' as possibile, i.e. they simply offload work to verification crates.

`/scripts`

Driver scripts to launch kernel as per local env.toml configuration file.
