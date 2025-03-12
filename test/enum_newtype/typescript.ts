/* This file is generated and managed by tsync */

export type Message<Params, Value> =
  | Message__Request<Params>
  | Message__Response<Value>;

type Message__Request<Params> = {
  type: "Request"} & Request<Params>
type Message__Response<Value> = {
  type: "Response"} & Response<Value>

export interface Request<Params> {
  id: string;
  method_type: string;
  params: Params;
}

export interface Response<Value> {
  id: string;
  result: Value;
}

export type CaptureConfigurationStruct =
  | CaptureConfigurationStruct__Video
  | CaptureConfigurationStruct__Redirect;

type CaptureConfigurationStruct__Video = {
  type: "video";
  height: number;
  width: number;
};
type CaptureConfigurationStruct__Redirect = {
  type: "redirect";
};

/** cases below were provided by joaoantoniocardoso on github in the discussion for issue #58 */
export type CaptureConfigurationNewtype =
  | CaptureConfigurationNewtype__Video
  | CaptureConfigurationNewtype__Redirect;

type CaptureConfigurationNewtype__Video = {
  type: "video"} & VideoCaptureConfiguration
type CaptureConfigurationNewtype__Redirect = {
  type: "redirect"} & RedirectCaptureConfiguration

export interface VideoCaptureConfiguration {
  height: number;
  width: number;
}

export interface RedirectCaptureConfiguration {
  [key: PropertyKey]: never;
}
