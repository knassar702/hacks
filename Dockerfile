FROM rust:1.31
WORKDIR /tools
COPY . /tools
RUN for i in $(find */Cargo.toml )
        do 
            n=$(dirname $i)
            cd $n
            echo "cd $n && cargo build"
            cargo install --verbose
            cd ../
        done
ENTRYPOINT ["sh"]
