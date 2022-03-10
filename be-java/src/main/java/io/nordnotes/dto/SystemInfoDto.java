package io.nordnotes.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AccessLevel;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

/**
 * Data transfer object for system information.
 */
@Getter
@Setter
@NoArgsConstructor(access = AccessLevel.PACKAGE)
public class SystemInfoDto {

  /** System name. */
  @JsonProperty("name")
  private String name;

  /** System version. */
  @JsonProperty("version")
  private String version;

  /** Legal note. */
  @JsonProperty("legalNote")
  private String legalNote;
}
