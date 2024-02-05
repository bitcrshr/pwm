package grpc_admin_v1beta1

import (
	"net"

	"github.com/bitcrshr/pwm/protos/compiled/go/admin_v1beta1"
	"google.golang.org/grpc"
)

type server struct {
	admin_v1beta1.UnimplementedAdminServiceServer
}

func Serve() error {
	lis, err := net.Listen("tcp", "localhost:8080")
	if err != nil {
		return err
	}

	grpcServer := grpc.NewServer()
	s := server{}

	admin_v1beta1.RegisterAdminServiceServer(grpcServer, &s)

	if err := grpcServer.Serve(lis); err != nil {
		return err
	}

	return nil
}
