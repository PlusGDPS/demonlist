use super::Page;
use crate::{model::user::User, permissions::Permissions, state::PointercrateState, ViewResult};
use actix_web::HttpResponse;
use actix_web_codegen::get;
use maud::{html, Markup, PreEscaped};

#[derive(Debug)]
struct Homepage {
    demonlist_team: Vec<User>,
    pointercrate_team: Vec<User>,
}

#[get("/")]
pub async fn index(state: PointercrateState) -> ViewResult<HttpResponse> {
    let mut connection = state.connection().await?;

    let demonlist_team = User::by_permission(Permissions::ListAdministrator, &mut connection).await?;
    let pointercrate_team = User::by_permission(Permissions::Administrator, &mut connection).await?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        Homepage {
            demonlist_team,
            pointercrate_team,
        }
        .render()
        .0,
    ))
}

impl Page for Homepage {
    fn title(&self) -> String {
        "Home".to_owned()
    }

    fn description(&self) -> String {
        "PlusGDPS Demonlist is the official demonlist for all the hardest rated demons that have been created and verified on PlusGDPS"
            .to_owned()
    }

    fn scripts(&self) -> Vec<&str> {
        vec!["js/home.js", "js/modules/tab.mjs"]
    }

    fn stylesheets(&self) -> Vec<&str> {
        vec!["css/home.css"]
    }

    fn body(&self) -> Markup {
        html! {
            div.information-banner.left {
                div.tab-display#information-tabs {
                    div.information {
                        div style = "display: flex; flex-flow: column;"{
                            h1 style="text-align: left; margin-top: 0px" {
                                "PlusGDPS Demonlist"
                            }
                            div.tab-content.tab-content-active data-tab-id ="1" {
                                "Welcome to the official PlusGDPS Demonlist! Here, you'll find all the hardest rated demons that have been created and verified on PlusGDPS!"
                            }
                        }
                    }
                    a.big.blue.hover.button.js-scroll-anim data-anim="fade" href = "/demonlist/"{
                        "Check it out"(PreEscaped("&nbsp;&nbsp;&nbsp;"))
                        i.fa.fa-arrow-right aria-hidden="true" {}
                    }
                }
            }

            aside.center.information-stripe {
                div.flex style="flex-wrap: wrap; align-items: center" {
                    span { "The Official PlusGDPS Demonlist!" }
                }
            }
            div.center.information-banner.right {
                div style = "flex-flow: column" {
                    h1#contact {
                        "Contact & Staff Members"
                    }
                    div.flex#about-inner {
                        div style = "flex-basis: 0; padding: 5px" {
                            h2 { "Demonlist Team: "}
                            h3 {
                                "The demonlist is managed by a large team of players lead by:"
                            }
                            div.flex.wrap style = "padding: 20px" {
                                @for member in &self.demonlist_team {
                                    h4 style="display: inline; margin: 5px" { (member.name()) }
                                }
                            }
                        }

                    }
                }
            }
        }
    }

    fn head(&self) -> Vec<Markup> {
        vec![html! {
            (PreEscaped(r#"
<style>
    .tab-active {
        color: #2F3135;
    }
</style>
<script type="application/ld+json">
  {
    "@context": "http://schema.org",
    "@type": "Organization",
    "name": "plusgdpsdemonlist",
    "description": "PlusGDPS Demonlist is the official demonlist for all the hardest rated demons that have been created and verified on PlusGDPS",
    "url": "https://pgdl.pluscraft.fr/",
    "logo": "https://pgdl.pluscraft.fr/static2/images/logo.png",
    "sameAs": [
      "https://twitter.com/PlusGDPS",
      "https://www.youtube.com/channel/UCb8B-6l4r30C4z8QCuFCVqQ"
    ]
  }
</script>
            "#))
        }]
    }
}
