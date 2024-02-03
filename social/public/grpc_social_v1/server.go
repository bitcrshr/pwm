package grpc_social_v1

import (
	"net"

	"github.com/bitcrshr/pwm/protos/compiled/go/social_v1"
	"google.golang.org/grpc"
)

type server struct {
	social_v1.UnimplementedSocialServiceServer
}

func Serve() error {
	lis, err := net.Listen("tcp", "localhost:8080")
	if err != nil {
		return err
	}

	grpcServer := grpc.NewServer()
	s := server{}

	social_v1.RegisterSocialServiceServer(grpcServer, &s)

	if err := grpcServer.Serve(lis); err != nil {
		return err
	}

	return nil
}
