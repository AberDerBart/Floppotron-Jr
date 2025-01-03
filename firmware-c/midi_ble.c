#include "midi_ble.h"
#include "midi_service_stream_handler.h"
#include "btstack.h"
#include "pico/cyw43_arch.h"
#include "pico/btstack_cyw43.h"

#include "midi-ble.h"

#define APP_AD_FLAGS 0x06
const uint8_t adv_data[] = {
    // Flags general discoverable
    0x02,
    BLUETOOTH_DATA_TYPE_FLAGS,
    APP_AD_FLAGS,
    // Service class list
    0x11,
    BLUETOOTH_DATA_TYPE_COMPLETE_LIST_OF_128_BIT_SERVICE_CLASS_UUIDS,
    0x00,
    0xc7,
    0xc4,
    0x4e,
    0xe3,
    0x6c,
    0x51,
    0xa7,
    0x33,
    0x4b,
    0xe8,
    0xed,
    0x5a,
    0x0e,
    0xb8,
    0x03,
};
const uint8_t adv_data_len = sizeof(adv_data);

const uint8_t scan_resp_data[] = {
    // Name
    0x0E,
    BLUETOOTH_DATA_TYPE_COMPLETE_LOCAL_NAME,
    'F',
    'l',
    'o',
    'p',
    'p',
    'o',
    't',
    'r',
    'o',
    'n',
    ' ',
    'J',
    'r',
};
const uint8_t scan_resp_data_len = sizeof(scan_resp_data);

static hci_con_handle_t con_handle = HCI_CON_HANDLE_INVALID;

void packet_handler(uint8_t packet_type, uint16_t channel, uint8_t *packet, uint16_t size)
{
  UNUSED(size);
  UNUSED(channel);
  bd_addr_t local_addr;
  uint8_t event_type;
  // bd_addr_t addr;
  // bd_addr_type_t addr_type;
  // uint8_t status;

  if (packet_type != HCI_EVENT_PACKET)
  {
    return;
  }

  event_type = hci_event_packet_get_type(packet);
  switch (event_type)
  {
  case BTSTACK_EVENT_STATE:
    if (btstack_event_state_get_state(packet) != HCI_STATE_WORKING)
    {
      return;
    }
    gap_local_bd_addr(local_addr);
    printf("BTstack up and running on %s.\n", bd_addr_to_str(local_addr));

    // setup advertisements
    uint16_t adv_int_min = 800;
    uint16_t adv_int_max = 800;
    uint8_t adv_type = 0;
    bd_addr_t null_addr;
    memset(null_addr, 0, 6);
    gap_advertisements_set_params(adv_int_min, adv_int_max, adv_type, 0, null_addr, 0x07, 0x00);
    assert(adv_data_len <= 31); // ble limitation
    gap_advertisements_set_data(adv_data_len, (uint8_t *)adv_data);
    assert(scan_resp_data_len <= 31); // ble limitation
    gap_scan_response_set_data(scan_resp_data_len, (uint8_t *)scan_resp_data);
    gap_advertisements_enable(1);

    break;
  case HCI_EVENT_DISCONNECTION_COMPLETE:
    printf("HCI_EVENT_DISCONNECTION_COMPLETE event\r\n");
    break;
  case HCI_EVENT_GATTSERVICE_META:
    switch (hci_event_gattservice_meta_get_subevent_code(packet))
    {
    case GATTSERVICE_SUBEVENT_SPP_SERVICE_CONNECTED:
      con_handle = gattservice_subevent_spp_service_connected_get_con_handle(packet);
      printf("GATTSERVICE_SUBEVENT_SPP_SERVICE_CONNECTED event\r\n");
      break;
    case GATTSERVICE_SUBEVENT_SPP_SERVICE_DISCONNECTED:
      printf("GATTSERVICE_SUBEVENT_SPP_SERVICE_DISCONNECTED event\r\n");
      con_handle = HCI_CON_HANDLE_INVALID;
      break;
    default:
      break;
    }
    break;
  case SM_EVENT_JUST_WORKS_REQUEST:
    printf("ble-midi2usbhost: Just Works requested\n");
    sm_just_works_confirm(sm_event_just_works_request_get_handle(packet));
    break;
  default:
    break;
  }
}

static btstack_packet_callback_registration_t sm_event_callback_registration;

void midi_ble_init()
{
  printf("init ble midi\n");

  if (cyw43_arch_init())
  {
    printf("failed to initialise cyw43_arch\n");
    return;
  }
  l2cap_init();

  sm_init();

  att_server_init(profile_data, NULL, NULL);
  // just works, legacy pairing
  sm_set_io_capabilities(IO_CAPABILITY_NO_INPUT_NO_OUTPUT);
  sm_set_authentication_requirements(SM_AUTHREQ_SECURE_CONNECTION | SM_AUTHREQ_BONDING);
  // register for SM events
  sm_event_callback_registration.callback = &packet_handler;
  sm_add_event_handler(&sm_event_callback_registration);
  midi_service_stream_init(packet_handler);

  hci_power_control(HCI_POWER_ON);
}

void midi_ble_write(uint8_t n_bytes, uint8_t *midi_stream_bytes)
{
  if (con_handle == HCI_CON_HANDLE_INVALID)
  {
    return;
  }

  midi_service_stream_write(con_handle, n_bytes, midi_stream_bytes);
}

bool midi_ble_try_read(struct midi_packet *packet)
{
  uint8_t buffer[128];
  uint8_t bytes_read = midi_service_stream_read(con_handle, 128, buffer, NULL);
  if (bytes_read == 0)
  {
    return false;
  }
  uint8_t index = 0;

  packet->status = buffer[0];
  while (!midi_is_status_byte(packet->status))
  {
    index++;
    if (index >= bytes_read - 2)
    {
      return false;
    }
    packet->status = buffer[index];
  }
  packet->b1 = buffer[index + 1];
  packet->b2 = buffer[index + 2];
  return true;
}