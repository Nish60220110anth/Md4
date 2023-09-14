buildbackend:
	@echo "Building backend..."
	@cd backend &&  cargo build --release && cp ./target/release/md4 ./..

buildfrontend:
	@echo "Building frontend..."
	@go build -o frontend main.go

build: buildbackend buildfrontend

runfrontend:
	@echo "Running frontend..."
	@./frontend

runall: buildbackend buildfrontend runfrontend

clean:
	@echo "Cleaning..."
	@rm backend/*.lock
	@rm -rf frontend backend/target
	@rm md4
	@rm out*.txt stat*.txt