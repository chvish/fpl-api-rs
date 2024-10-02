# FPL API
- Rust bindings for the Fantasy Premier League API

## Example

```
        let client = FPLClient::new();
        let manager = client.get_manager_details("3332624").await.unwrap();
        assert_eq!(manager.player_first_name, "Vishal".to_string());
        assert_eq!(manager.name, "Gakpo Ke Gunde".to_string());

        let gw_team = client
            .get_manager_team_for_gw("3332624", "4")
            .await
            .unwrap();
        assert_eq!(gw_team.active_chip, None);

        let transfers = client.get_manager_transfers("3332624").await.unwrap();
        assert!(transfers.len() > 0);

```
