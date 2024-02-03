// @generated by protobuf-ts 2.9.3
// @generated from protobuf file "service.proto" (package "config.v1", syntax proto3)
// tslint:disable
import { ResetPalWorldSettingsResponse } from "./settings";
import { ResetPalWorldSettingsRequest } from "./settings";
import { UpdatePalWorldSettingsResponse } from "./settings";
import { UpdatePalWorldSettingsRequest } from "./settings";
import { GetPalWorldSettingsResponse } from "./settings";
import { GetPalWorldSettingsRequest } from "./settings";
import { ServiceType } from "@protobuf-ts/runtime-rpc";
/**
 * @generated ServiceType for protobuf service config.v1.ConfigService
 */
export const ConfigService = new ServiceType("config.v1.ConfigService", [
    { name: "GetPalWorldSettings", options: {}, I: GetPalWorldSettingsRequest, O: GetPalWorldSettingsResponse },
    { name: "UpdatePalWorldSettings", options: {}, I: UpdatePalWorldSettingsRequest, O: UpdatePalWorldSettingsResponse },
    { name: "ResetPalWorldSettings", options: {}, I: ResetPalWorldSettingsRequest, O: ResetPalWorldSettingsResponse }
]);
