import { BasecoatClient as backend_client } from "./proto/basecoat_grpc_web_pb";
import { GetSystemInfoRequest } from "./proto/basecoat_transport_pb";

class BasecoatClient {
  constructor() {
    let url = location.protocol + "//" + location.host;
    this.client = new backend_client(url, null, null);
  }

  get_system_info() {
    let getSystemInfoRequest = new GetSystemInfoRequest();
    return new Promise((resolve, reject) => {
      this.client.getSystemInfo(
        getSystemInfoRequest,
        {},
        function (err, response) {
          if (err) {
            reject(err);
            return;
          }
          let systemInfo = {
            commit: response.getCommit(),
            debug_enabled: response.getDebugEnabled(),
            semver: response.getSemver(),
          };
          resolve(systemInfo);
        }
      );
    });
  }
}

export let client = new BasecoatClient();
