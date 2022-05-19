<script>
  import { BasecoatClient } from "../proto/basecoat_grpc_web_pb";
  import { GetSystemInfoRequest } from "../proto/basecoat_transport_pb";

  url = location.protocol + "//" + location.host;
  var basecoat_client = new BasecoatClient(url, null, null);
  let get_system_info_request = new GetSystemInfoRequest();
  let info = basecoat_client.getSystemInfo(get_system_info_request);
  console.log(info);
</script>

<footer>
  <hr class="border-t border-gray-500 mb-5" />
  <div class="text-gray-500 font-light">test</div>
</footer>
